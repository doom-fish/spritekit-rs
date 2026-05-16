use spritekit::{CGSize, ThreeDNode};

fn main() {
    let node = ThreeDNode::with_viewport_size(CGSize::new(128.0, 96.0)).expect("3d node");
    node.set_empty_scene();
    node.set_scene_time(1.5);
    node.set_playing(true);
    node.set_loops(false);
    node.set_autoenables_default_lighting(true);

    let viewport = node.viewport_size();
    assert!((viewport.width - 128.0).abs() < 1e-6);
    assert!((viewport.height - 96.0).abs() < 1e-6);
    assert!(node.has_scene());
    assert!((node.scene_time() - 1.5).abs() < 1e-6);
    assert!(node.is_playing());
    assert!(!node.loops());
    assert!(node.autoenables_default_lighting());

    println!("3d node basic ok");
}
