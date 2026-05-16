use spritekit::{CGSize, ThreeDNode};

#[test]
fn three_d_node_tracks_scene_and_playback_state() {
    let node = ThreeDNode::with_viewport_size(CGSize::new(40.0, 30.0)).expect("3d node");
    node.set_empty_scene();
    node.set_scene_time(2.5);
    node.set_playing(true);
    node.set_loops(true);
    node.set_autoenables_default_lighting(true);

    let viewport = node.viewport_size();
    assert!((viewport.width - 40.0).abs() < 1e-6);
    assert!((viewport.height - 30.0).abs() < 1e-6);
    assert!(node.has_scene());
    assert!((node.scene_time() - 2.5).abs() < 1e-6);
    assert!(node.is_playing());
    assert!(node.loops());
    assert!(node.autoenables_default_lighting());
}
