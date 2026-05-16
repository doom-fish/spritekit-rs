use spritekit::{CGPoint, CGVector, CGSize, BlendMode, EmitterNode, Texture};

fn main() {
    let emitter = EmitterNode::new().expect("emitter");
    let texture = Texture::from_rgba_bytes(&[255, 255, 255, 255], 1, 1).expect("texture");

    emitter.set_particle_texture(Some(&texture));
    emitter.set_particle_blend_mode(BlendMode::Add);
    emitter.set_particle_position(CGPoint::new(4.0, 6.0));
    emitter.set_particle_position_range(CGVector::new(10.0, 20.0));
    emitter.set_particle_speed(50.0);
    emitter.set_particle_birth_rate(8.0);
    emitter.set_num_particles_to_emit(32);
    emitter.set_particle_lifetime(2.0);
    emitter.set_particle_size(CGSize::new(3.0, 5.0));
    emitter.set_particle_scale(1.25);
    emitter.advance_simulation_time(0.2);

    let position = emitter.particle_position();
    let size = emitter.particle_size();
    assert!(emitter.particle_texture().is_some());
    assert_eq!(emitter.particle_blend_mode(), BlendMode::Add);
    assert!((position.x - 4.0).abs() < 1e-6);
    assert!((position.y - 6.0).abs() < 1e-6);
    assert!((size.width - 3.0).abs() < 1e-6);
    assert!((size.height - 5.0).abs() < 1e-6);
    assert_eq!(emitter.num_particles_to_emit(), 32);

    println!("emitter node particles ok");
}
