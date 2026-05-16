use spritekit::{CGPoint, CGSize, NodeExt, Shader, SpriteNode};

fn main() {
    let shader = Shader::with_source("void main() { gl_FragColor = vec4(1.0); }").expect("shader");
    shader.add_float_uniform("u_gain", 0.75);

    let sprite = SpriteNode::with_color(spritekit::Color::white(), CGSize::new(24.0, 24.0))
        .expect("sprite");
    sprite.set_position(CGPoint::new(5.0, 5.0));
    sprite.set_shader(Some(&shader));

    let gain = shader.float_uniform_named("u_gain").expect("u_gain");
    assert_eq!(shader.uniform_count(), 1);
    assert!((gain - 0.75).abs() < 1e-6);
    assert!(sprite.shader().is_some());

    println!("shader uniforms ok");
}
