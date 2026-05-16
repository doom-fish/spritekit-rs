use spritekit::{CGPoint, CGSize, VideoNode};

#[test]
fn video_node_round_trips_geometry() {
    let video = VideoNode::new().expect("video node");
    video.set_size(CGSize::new(64.0, 36.0));
    video.set_anchor_point(CGPoint::new(0.5, 0.0));

    let size = video.size();
    let anchor = video.anchor_point();
    assert!((size.width - 64.0).abs() < 1e-6);
    assert!((size.height - 36.0).abs() < 1e-6);
    assert!((anchor.x - 0.5).abs() < 1e-6);
    assert!(anchor.y.abs() < 1e-6);
}
