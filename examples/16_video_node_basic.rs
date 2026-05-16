use spritekit::{CGPoint, CGSize, VideoNode};

fn main() {
    let video = VideoNode::new().expect("video node");
    video.set_size(CGSize::new(80.0, 45.0));
    video.set_anchor_point(CGPoint::new(0.0, 0.0));
    video.play();
    video.pause();

    let size = video.size();
    let anchor = video.anchor_point();
    assert!((size.width - 80.0).abs() < 1e-6);
    assert!((size.height - 45.0).abs() < 1e-6);
    assert!(anchor.x.abs() < 1e-6);
    assert!(anchor.y.abs() < 1e-6);

    println!("video node basic ok");
}
