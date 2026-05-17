use spritekit::{CGSize, Shader, SpriteNode};

#[test]
fn shader_tracks_uniforms() {
    let shader = Shader::with_source("void main() { gl_FragColor = vec4(1.0); }").expect("shader");
    let sprite =
        SpriteNode::with_color(spritekit::Color::white(), CGSize::new(10.0, 10.0)).expect("sprite");

    shader.add_float_uniform("u_amount", 0.5);
    sprite.set_shader(Some(&shader));

    let amount = shader.float_uniform_named("u_amount").expect("u_amount");
    assert_eq!(shader.uniform_count(), 1);
    assert!((amount - 0.5).abs() < 1e-6);
    assert!(sprite.shader().is_some());
}
