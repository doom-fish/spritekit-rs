use spritekit::{CGPoint, CGRect, CGSize, Color, SpriteNode, Texture};

#[test]
fn sprite_node_tracks_textures_and_masks() {
    let texture = Texture::from_rgba_bytes(&[255, 0, 0, 255], 1, 1).expect("texture");
    let sprite = SpriteNode::with_color(Color::red(), CGSize::new(20.0, 20.0)).expect("sprite");

    sprite.set_texture(Some(&texture));
    sprite.set_normal_texture(Some(&texture));
    sprite.set_anchor_point(CGPoint::new(0.2, 0.8));
    sprite.set_center_rect(CGRect::new(0.2, 0.2, 0.6, 0.6));
    sprite.set_lighting_bitmask(7);
    sprite.set_shadow_cast_bitmask(3);
    sprite.set_shadowed_bitmask(5);

    let anchor = sprite.anchor_point();
    let center = sprite.center_rect();
    assert!(sprite.texture().is_some());
    assert!(sprite.normal_texture().is_some());
    assert!((anchor.x - 0.2).abs() < 1e-6);
    assert!((anchor.y - 0.8).abs() < 1e-6);
    assert!((center.origin.x - 0.2).abs() < 1e-6);
    assert!((center.origin.y - 0.2).abs() < 1e-6);
    assert!((center.size.width - 0.6).abs() < 1e-6);
    assert!((center.size.height - 0.6).abs() < 1e-6);
    assert_eq!(sprite.lighting_bitmask(), 7);
    assert_eq!(sprite.shadow_cast_bitmask(), 3);
    assert_eq!(sprite.shadowed_bitmask(), 5);
}
