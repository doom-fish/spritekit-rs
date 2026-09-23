use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use spritekit::{Action, Node, NodeExt};

mod common;

use common::{autoreleasepool, Stage};

fn stage_with_node() -> (Stage, Node) {
    let stage = Stage::new(64.0, 64.0);
    let node = Node::new().expect("node");
    stage.scene.add_child(&node).expect("add node");
    (stage, node)
}

#[test]
fn completion_runs_once_after_the_action_finishes() {
    let (mut stage, node) = stage_with_node();
    let completions = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&completions);
    node.run_action_with_completion(&Action::wait(0.05).expect("wait"), move || {
        counter.fetch_add(1, Ordering::SeqCst);
    });

    stage.step(1);
    assert_eq!(completions.load(Ordering::SeqCst), 0);
    stage.step(10);
    assert_eq!(completions.load(Ordering::SeqCst), 1);
    assert!(!node.has_actions());
    stage.step(5);
    assert_eq!(completions.load(Ordering::SeqCst), 1);
    assert_eq!(Arc::strong_count(&completions), 1);
}

#[test]
fn completion_is_released_without_running_when_the_action_is_removed() {
    let (mut stage, node) = stage_with_node();
    let completions = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&completions);
    node.run_action_with_completion(&Action::wait(10.0).expect("wait"), move || {
        counter.fetch_add(1, Ordering::SeqCst);
    });
    stage.step(2);
    assert_eq!(Arc::strong_count(&completions), 2);

    autoreleasepool(|| node.remove_all_actions());
    stage.step(2);
    assert_eq!(completions.load(Ordering::SeqCst), 0);
    assert_eq!(Arc::strong_count(&completions), 1);
}

#[test]
fn run_block_action_runs_its_closure_and_releases_it() {
    let (mut stage, node) = stage_with_node();
    let runs = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&runs);
    autoreleasepool(|| {
        let block = Action::run_block(move || {
            counter.fetch_add(1, Ordering::SeqCst);
        })
        .expect("block action");
        let sequence =
            Action::sequence(&[&Action::wait(0.02).expect("wait"), &block]).expect("sequence");
        node.run_action(&sequence);
    });

    stage.step(1);
    assert_eq!(runs.load(Ordering::SeqCst), 0);
    stage.step(6);
    assert_eq!(runs.load(Ordering::SeqCst), 1);
    assert!(!node.has_actions());
    assert_eq!(Arc::strong_count(&runs), 1);
}

#[test]
fn custom_action_receives_the_running_node_and_elapsed_time() {
    let (mut stage, node) = stage_with_node();
    node.set_name("mover");
    let samples: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(Vec::new()));
    let names: Arc<Mutex<Vec<Option<String>>>> = Arc::new(Mutex::new(Vec::new()));
    let recorded_samples = Arc::clone(&samples);
    let recorded_names = Arc::clone(&names);
    autoreleasepool(|| {
        let custom = Action::custom(0.1, move |target: &Node, elapsed| {
            recorded_names.lock().expect("names").push(target.name());
            recorded_samples.lock().expect("samples").push(elapsed);
        })
        .expect("custom action");
        node.run_action(&custom);
    });

    stage.step(12);
    let samples = samples.lock().expect("samples").clone();
    assert!(samples.len() >= 2, "{samples:?}");
    assert!(
        samples.windows(2).all(|pair| pair[0] <= pair[1]),
        "{samples:?}"
    );
    assert!((samples.last().copied().expect("last sample") - 0.1).abs() < 1e-6);
    assert!(names
        .lock()
        .expect("names")
        .iter()
        .all(|name| name.as_deref() == Some("mover")));
    assert!(!node.has_actions());
    assert_eq!(Arc::strong_count(&names), 1);
}

#[test]
fn keyed_actions_can_be_looked_up_replaced_and_removed() {
    let (mut stage, node) = stage_with_node();
    let spin =
        Action::repeat_forever(&Action::rotate_by(1.0, 1.0).expect("rotate")).expect("forever");
    node.run_action_with_key(&spin, "spin").expect("run keyed");
    node.run_action_with_key(&Action::wait(10.0).expect("wait"), "idle")
        .expect("run second key");
    stage.step(2);

    assert!(node.action_for_key("spin").is_some());
    assert!(node.action_for_key("idle").is_some());
    assert!(node.action_for_key("missing").is_none());
    assert!(node.action_for_key("bad\0key").is_none());

    let error = node
        .run_action_with_key(&spin, "bad\0key")
        .expect_err("keys with NUL bytes are rejected");
    assert!(error.to_string().contains("NUL"), "{error}");

    node.remove_action_for_key("spin");
    assert!(node.action_for_key("spin").is_none());
    assert!(node.has_actions());

    node.run_action_with_key(&Action::wait(0.01).expect("wait"), "idle")
        .expect("replace keyed action");
    stage.step(4);
    assert!(node.action_for_key("idle").is_none());
    assert!(!node.has_actions());
}
