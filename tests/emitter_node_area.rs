use spritekit::{BlendMode, CGPoint, CGSize, EmitterNode, Texture};

#[test]
fn emitter_node_round_trips_particle_state() {
    let emitter = EmitterNode::new().expect("emitter");
    let texture = Texture::from_rgba_bytes(&[255, 255, 255, 255], 1, 1).expect("texture");

    emitter.set_particle_texture(Some(&texture));
    emitter.set_particle_blend_mode(BlendMode::Screen);
    emitter.set_particle_position(CGPoint::new(2.0, 3.0));
    emitter.set_particle_size(CGSize::new(4.0, 5.0));
    emitter.set_num_particles_to_emit(12);

    let position = emitter.particle_position();
    let size = emitter.particle_size();
    assert!(emitter.particle_texture().is_some());
    assert_eq!(emitter.particle_blend_mode(), BlendMode::Screen);
    assert!((position.x - 2.0).abs() < 1e-6);
    assert!((position.y - 3.0).abs() < 1e-6);
    assert!((size.width - 4.0).abs() < 1e-6);
    assert!((size.height - 5.0).abs() < 1e-6);
    assert_eq!(emitter.num_particles_to_emit(), 12);
}
