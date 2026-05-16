use spritekit::{CGPoint, CGRect, CGSize, Scene, View};

fn main() {
    let view = View::with_frame(CGRect::new(0.0, 0.0, 320.0, 240.0)).expect("view");
    let scene = Scene::with_size(CGSize::new(320.0, 240.0)).expect("scene");

    view.set_shows_fps(true);
    view.set_preferred_frames_per_second(60);
    view.present_scene(Some(&scene));

    assert!(view.scene().is_some());
    assert!(scene.view().is_some());
    assert_eq!(view.preferred_frames_per_second(), 60);
    assert!(view.shows_fps());

    let in_scene = scene.convert_point_from_view(CGPoint::new(10.0, 20.0));
    let back_to_view = scene.convert_point_to_view(in_scene);
    assert!(back_to_view.x.is_finite());
    assert!(back_to_view.y.is_finite());

    println!("view present scene ok");
}
