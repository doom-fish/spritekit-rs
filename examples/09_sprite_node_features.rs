use spritekit::{CGPoint, CGRect, CGSize, Color, SpriteNode, Texture};

fn main() {
    let texture = Texture::from_rgba_bytes(&[255, 0, 0, 255], 1, 1).expect("texture");
    let sprite = SpriteNode::with_color(Color::red(), CGSize::new(32.0, 32.0)).expect("sprite");

    sprite.set_texture(Some(&texture));
    sprite.set_normal_texture(Some(&texture));
    sprite.set_anchor_point(CGPoint::new(0.25, 0.75));
    sprite.set_lighting_bitmask(0b0011);
    sprite.set_shadow_cast_bitmask(0b0100);
    sprite.set_shadowed_bitmask(0b1000);
    sprite.set_center_rect(CGRect::new(0.25, 0.25, 0.5, 0.5));
    sprite.scale_to_size(CGSize::new(48.0, 24.0));

    let anchor = sprite.anchor_point();
    let size = sprite.size();
    assert!(sprite.texture().is_some());
    assert!(sprite.normal_texture().is_some());
    assert!((anchor.x - 0.25).abs() < 1e-6);
    assert!((anchor.y - 0.75).abs() < 1e-6);
    assert_eq!(sprite.lighting_bitmask(), 0b0011);
    assert_eq!(sprite.shadow_cast_bitmask(), 0b0100);
    assert_eq!(sprite.shadowed_bitmask(), 0b1000);
    assert!((size.width - 48.0).abs() < 1e-6);
    assert!((size.height - 24.0).abs() < 1e-6);

    println!("sprite node features ok");
}
