use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use spritekit::{SceneDelegate, SceneDelegateCallbacks};

mod common;

use common::{add_overlapping_bodies, autoreleasepool, counting_delegate, Stage};

fn has_contact_delegate(stage: &Stage) -> bool {
    autoreleasepool(|| stage.scene.physics_world().has_contact_delegate())
}

#[test]
fn contact_delegate_receives_contacts_from_the_simulation() {
    let mut stage = Stage::new(128.0, 128.0);
    add_overlapping_bodies(&stage);
    let begins = Arc::new(AtomicUsize::new(0));
    let delegate = counting_delegate(&begins);
    stage
        .scene
        .physics_world()
        .set_contact_delegate(Some(&delegate));
    stage.step(10);
    assert!(begins.load(Ordering::SeqCst) > 0);
    autoreleasepool(|| stage.scene.physics_world().set_contact_delegate(None));
}

#[test]
fn dropping_a_contact_delegate_while_it_is_set_keeps_the_world_safe() {
    let mut stage = Stage::new(128.0, 128.0);
    add_overlapping_bodies(&stage);
    let begins = Arc::new(AtomicUsize::new(0));
    autoreleasepool(|| {
        let delegate = counting_delegate(&begins);
        stage
            .scene
            .physics_world()
            .set_contact_delegate(Some(&delegate));
    });

    assert!(has_contact_delegate(&stage));
    stage.step(10);
    assert_eq!(begins.load(Ordering::SeqCst), 0);
    assert!(has_contact_delegate(&stage));
    assert_eq!(Arc::strong_count(&begins), 2);

    autoreleasepool(|| stage.scene.physics_world().set_contact_delegate(None));
    assert!(!has_contact_delegate(&stage));
    assert_eq!(
        Arc::strong_count(&begins),
        2,
        "a cleared delegate stays alive while SpriteKit may still be using it"
    );
    stage.step(5);
    assert_eq!(begins.load(Ordering::SeqCst), 0);
}

#[test]
fn replacing_a_contact_delegate_keeps_the_previous_one_inactive() {
    let mut stage = Stage::new(128.0, 128.0);
    add_overlapping_bodies(&stage);
    let first = Arc::new(AtomicUsize::new(0));
    let second = Arc::new(AtomicUsize::new(0));
    autoreleasepool(|| {
        let delegate = counting_delegate(&first);
        stage
            .scene
            .physics_world()
            .set_contact_delegate(Some(&delegate));
    });
    let replacement = counting_delegate(&second);
    autoreleasepool(|| {
        stage
            .scene
            .physics_world()
            .set_contact_delegate(Some(&replacement));
    });
    assert_eq!(Arc::strong_count(&first), 2);

    stage.step(10);
    assert_eq!(first.load(Ordering::SeqCst), 0);
    assert!(second.load(Ordering::SeqCst) > 0);
    autoreleasepool(|| stage.scene.physics_world().set_contact_delegate(None));
}

#[test]
fn scene_delegate_receives_every_frame_phase_in_order() {
    let mut stage = Stage::new(64.0, 64.0);
    let events: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
    let record = |label: &'static str| {
        let events = Arc::clone(&events);
        move || events.lock().expect("events").push(label)
    };
    let update_events = Arc::clone(&events);
    let delegate = SceneDelegate::from_callbacks(
        SceneDelegateCallbacks::new()
            .on_update(move |time| {
                assert!(time.is_finite());
                update_events.lock().expect("events").push("update");
            })
            .on_did_evaluate_actions(record("did_evaluate_actions"))
            .on_did_simulate_physics(record("did_simulate_physics"))
            .on_did_apply_constraints(record("did_apply_constraints"))
            .on_did_finish_update(record("did_finish_update")),
    )
    .expect("scene delegate");
    stage.scene.set_delegate(Some(&delegate));

    stage.step(2);
    let phases = [
        "update",
        "did_evaluate_actions",
        "did_simulate_physics",
        "did_apply_constraints",
        "did_finish_update",
    ];
    let expected: Vec<&str> = phases.iter().chain(phases.iter()).copied().collect();
    assert_eq!(*events.lock().expect("events"), expected);
}

#[test]
fn dropping_a_scene_delegate_while_it_is_set_stops_callbacks() {
    let mut stage = Stage::new(64.0, 64.0);
    let updates = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&updates);
    let delegate = SceneDelegate::with_update(move |_| {
        counter.fetch_add(1, Ordering::SeqCst);
    })
    .expect("scene delegate");
    stage.scene.set_delegate(Some(&delegate));
    stage.step(2);
    assert_eq!(updates.load(Ordering::SeqCst), 2);

    autoreleasepool(|| drop(delegate));
    stage.step(3);
    assert_eq!(updates.load(Ordering::SeqCst), 2);
    assert!(!autoreleasepool(|| stage.scene.has_delegate()));
    assert_eq!(Arc::strong_count(&updates), 1);
}
