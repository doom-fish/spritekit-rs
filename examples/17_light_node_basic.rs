use spritekit::{CGSize, Color, LightNode, SpriteNode};

fn main() {
    let light = LightNode::new().expect("light node");
    let sprite = SpriteNode::with_color(Color::white(), CGSize::new(16.0, 16.0)).expect("sprite");

    light.set_enabled(true);
    light.set_falloff(1.5);
    light.set_category_bitmask(0b0101);
    light.set_light_color(Color::white());
    light.set_ambient_color(Color::rgb(0.1, 0.1, 0.1));
    light.set_shadow_color(Color::black());
    sprite.set_lighting_bitmask(0b0101);

    assert!(light.is_enabled());
    assert!((light.falloff() - 1.5).abs() < f64::EPSILON);
    assert_eq!(light.category_bitmask(), 0b0101);
    assert_eq!(sprite.lighting_bitmask(), 0b0101);

    println!("light node basic ok");
}
