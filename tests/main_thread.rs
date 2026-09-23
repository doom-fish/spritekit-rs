use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use spritekit::{
    CGRect, CGSize, CameraNode, Color, NodeExt, Scene, SceneDelegate, SpriteNode, Transition,
    TransitionDirection, View, ViewDelegate,
};

mod common;

use common::{add_overlapping_bodies, autoreleasepool, counting_delegate, drain_main_queue, Stage};

fn view() -> View {
    View::with_frame(CGRect::new(0.0, 0.0, 160.0, 90.0)).expect("view on the main thread")
}

fn view_presents_scenes_and_round_trips_properties() {
    let view = view();
    let scene = Scene::with_size(CGSize::new(160.0, 90.0)).expect("scene");
    view.set_shows_fps(true);
    view.set_shows_node_count(true);
    view.set_ignores_sibling_order(true);
    view.set_preferred_frames_per_second(30);
    view.present_scene(Some(&scene));

    assert!(view.scene().is_some());
    assert!(scene.view().is_some());
    assert!(view.shows_fps());
    assert!(view.shows_node_count());
    assert!(view.ignores_sibling_order());
    assert_eq!(view.preferred_frames_per_second(), 30);

    view.present_scene(None);
    assert!(view.scene().is_none());
}

fn view_presents_scenes_with_transitions_and_delegates() {
    let view = view();
    let scene = Scene::with_size(CGSize::new(160.0, 90.0)).expect("scene");
    let next_scene = Scene::with_size(CGSize::new(160.0, 90.0)).expect("next scene");
    let camera = CameraNode::new().expect("camera");
    scene.set_camera(Some(&camera));
    assert!(scene.camera().is_some());
    assert_eq!(camera.contained_node_count(), 0);

    let scene_delegate = SceneDelegate::with_update(|_| {}).expect("scene delegate");
    scene.set_delegate(Some(&scene_delegate));
    assert!(scene.has_delegate());

    let view_delegate = ViewDelegate::with_should_render(|_| true).expect("view delegate");
    view.set_delegate(Some(&view_delegate));
    assert!(view.has_delegate());

    let transition = Transition::push(TransitionDirection::Left, 0.25).expect("transition");
    transition.set_pauses_incoming_scene(true);
    transition.set_pauses_outgoing_scene(false);
    assert!(transition.pauses_incoming_scene());
    assert!(!transition.pauses_outgoing_scene());

    view.present_scene(Some(&scene));
    view.present_scene_with_transition(&next_scene, &transition);
    assert!(view.scene().is_some());
    assert!(next_scene.view().is_some());
    view.set_delegate(None);
    assert!(!view.has_delegate());
}

fn dropping_a_view_delegate_while_it_is_set_is_safe() {
    let view = view();
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    autoreleasepool(|| {
        let delegate = ViewDelegate::with_should_render(move |_| {
            counter.fetch_add(1, Ordering::SeqCst);
            true
        })
        .expect("view delegate");
        view.set_delegate(Some(&delegate));
        assert!(view.has_delegate());
    });

    assert!(!autoreleasepool(|| view.has_delegate()));
    assert_eq!(Arc::strong_count(&calls), 1);
    let scene = Scene::with_size(CGSize::new(160.0, 90.0)).expect("scene");
    view.present_scene(Some(&scene));
}

fn texture_from_node_uses_the_view() {
    let view = view();
    let scene = Scene::with_size(CGSize::new(160.0, 90.0)).expect("scene");
    let sprite = SpriteNode::with_color(Color::red(), CGSize::new(8.0, 8.0)).expect("sprite");
    scene.add_child(&sprite).expect("add sprite");
    view.present_scene(Some(&scene));
    if let Some(texture) = view.texture_from_node(&sprite) {
        let size = texture.size();
        assert!(size.width > 0.0 && size.height > 0.0);
    }
}

fn view_creation_requires_the_main_thread() {
    let view = view();
    let scene = Scene::with_size(CGSize::new(160.0, 90.0)).expect("scene");
    view.present_scene(Some(&scene));
    assert!(scene.view().is_some());
    let error = std::thread::spawn(|| {
        View::with_frame(CGRect::new(0.0, 0.0, 8.0, 8.0))
            .map(drop)
            .expect_err("off-main views must be rejected")
    })
    .join()
    .expect("worker thread");
    assert!(error.to_string().contains("main thread"), "{error}");
}

fn dropping_the_scene_releases_a_contact_delegate_that_is_still_set() {
    let begins = Arc::new(AtomicUsize::new(0));
    autoreleasepool(|| {
        let mut stage = Stage::new(128.0, 128.0);
        add_overlapping_bodies(&stage);
        let delegate = counting_delegate(&begins);
        stage
            .scene
            .physics_world()
            .set_contact_delegate(Some(&delegate));
        stage.step(3);
        drop(stage);
        drop(delegate);
    });
    drain_main_queue();
    assert!(begins.load(Ordering::SeqCst) > 0);
    assert_eq!(Arc::strong_count(&begins), 1);
}

fn main() {
    let tests: [(&str, fn()); 6] = [
        (
            "view_presents_scenes_and_round_trips_properties",
            view_presents_scenes_and_round_trips_properties,
        ),
        (
            "view_presents_scenes_with_transitions_and_delegates",
            view_presents_scenes_with_transitions_and_delegates,
        ),
        (
            "dropping_a_view_delegate_while_it_is_set_is_safe",
            dropping_a_view_delegate_while_it_is_set_is_safe,
        ),
        (
            "texture_from_node_uses_the_view",
            texture_from_node_uses_the_view,
        ),
        (
            "view_creation_requires_the_main_thread",
            view_creation_requires_the_main_thread,
        ),
        (
            "dropping_the_scene_releases_a_contact_delegate_that_is_still_set",
            dropping_the_scene_releases_a_contact_delegate_that_is_still_set,
        ),
    ];
    for (name, test) in tests {
        autoreleasepool(test);
        println!("test {name} ... ok");
    }
    println!("test result: ok. {} passed", tests.len());
}
