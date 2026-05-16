use spritekit::{CGRect, CGSize, Scene, View};

#[test]
fn view_can_present_scene_headlessly() {
    let view = View::with_frame(CGRect::new(0.0, 0.0, 160.0, 90.0)).expect("view");
    let scene = Scene::with_size(CGSize::new(160.0, 90.0)).expect("scene");

    view.set_shows_fps(true);
    view.present_scene(Some(&scene));

    assert!(view.scene().is_some());
    assert!(scene.view().is_some());
    assert!(view.shows_fps());
}
