use spritekit::{Color, LightNode};

#[test]
fn light_node_round_trips_core_properties() {
    let light = LightNode::new().expect("light node");
    light.set_enabled(true);
    light.set_falloff(2.0);
    light.set_category_bitmask(9);
    light.set_light_color(Color::white());
    light.set_shadow_color(Color::black());

    assert!(light.is_enabled());
    assert!((light.falloff() - 2.0).abs() < f64::EPSILON);
    assert_eq!(light.category_bitmask(), 9);
}
