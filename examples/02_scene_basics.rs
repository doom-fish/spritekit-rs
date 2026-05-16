use spritekit::{CGPoint, CGSize, Color, Scene, SceneScaleMode};

fn main() {
    let scene = Scene::with_size(CGSize::new(320.0, 240.0)).expect("scene");
    scene.set_scale_mode(SceneScaleMode::AspectFill);
    scene.set_background_color(Color::blue());
    scene.set_anchor_point(CGPoint::new(0.25, 0.75));

    let size = scene.size();
    let anchor = scene.anchor_point();
    assert!((size.width - 320.0).abs() < 1e-6);
    assert!((size.height - 240.0).abs() < 1e-6);
    assert!((anchor.x - 0.25).abs() < 1e-6);
    assert!((anchor.y - 0.75).abs() < 1e-6);
    assert_eq!(scene.scale_mode(), SceneScaleMode::AspectFill);

    println!("scene basics ok");
}
