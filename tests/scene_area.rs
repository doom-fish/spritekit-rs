use spritekit::{CGPoint, CGSize, Scene, SceneScaleMode};

#[test]
fn scene_round_trips_size_and_anchor() {
    let scene = Scene::with_size(CGSize::new(200.0, 150.0)).expect("scene");
    scene.set_scale_mode(SceneScaleMode::ResizeFill);
    scene.set_anchor_point(CGPoint::new(0.1, 0.9));

    let size = scene.size();
    let anchor = scene.anchor_point();
    assert!((size.width - 200.0).abs() < 1e-6);
    assert!((size.height - 150.0).abs() < 1e-6);
    assert_eq!(scene.scale_mode(), SceneScaleMode::ResizeFill);
    assert!((anchor.x - 0.1).abs() < 1e-6);
    assert!((anchor.y - 0.9).abs() < 1e-6);
}
