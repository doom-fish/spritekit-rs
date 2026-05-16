import SpriteKit

@_cdecl("sk_emitter_node_new")
public func sk_emitter_node_new() -> UnsafeMutableRawPointer? {
    skRetain(SKEmitterNode())
}

@_cdecl("sk_emitter_node_advance_simulation_time")
public func sk_emitter_node_advance_simulation_time(_ nodeHandle: UnsafeMutableRawPointer?, _ seconds: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.advanceSimulationTime(seconds)
}

@_cdecl("sk_emitter_node_reset_simulation")
public func sk_emitter_node_reset_simulation(_ nodeHandle: UnsafeMutableRawPointer?) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.resetSimulation()
}

@_cdecl("sk_emitter_node_get_particle_texture")
public func sk_emitter_node_get_particle_texture(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SKEmitterNode = skBorrow(nodeHandle), let texture = node.particleTexture else { return nil }
    return skRetain(texture)
}

@_cdecl("sk_emitter_node_set_particle_texture")
public func sk_emitter_node_set_particle_texture(_ nodeHandle: UnsafeMutableRawPointer?, _ textureHandle: UnsafeMutableRawPointer?) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    let texture: SKTexture? = skBorrow(textureHandle)
    node.particleTexture = texture
}

@_cdecl("sk_emitter_node_get_particle_blend_mode")
public func sk_emitter_node_get_particle_blend_mode(_ nodeHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Int32(node.particleBlendMode.rawValue)
}

@_cdecl("sk_emitter_node_set_particle_blend_mode")
public func sk_emitter_node_set_particle_blend_mode(_ nodeHandle: UnsafeMutableRawPointer?, _ mode: Int32) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particleBlendMode = skBlendMode(mode)
}

@_cdecl("sk_emitter_node_set_particle_color")
public func sk_emitter_node_set_particle_color(_ nodeHandle: UnsafeMutableRawPointer?, _ r: Float, _ g: Float, _ b: Float, _ a: Float) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particleColor = skMakeColor(r: r, g: g, b: b, a: a)
}

@_cdecl("sk_emitter_node_get_particle_position_x")
public func sk_emitter_node_get_particle_position_x(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particlePosition.x)
}

@_cdecl("sk_emitter_node_get_particle_position_y")
public func sk_emitter_node_get_particle_position_y(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particlePosition.y)
}

@_cdecl("sk_emitter_node_set_particle_position")
public func sk_emitter_node_set_particle_position(_ nodeHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particlePosition = CGPoint(x: x, y: y)
}

@_cdecl("sk_emitter_node_get_particle_position_range_dx")
public func sk_emitter_node_get_particle_position_range_dx(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particlePositionRange.dx)
}

@_cdecl("sk_emitter_node_get_particle_position_range_dy")
public func sk_emitter_node_get_particle_position_range_dy(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particlePositionRange.dy)
}

@_cdecl("sk_emitter_node_set_particle_position_range")
public func sk_emitter_node_set_particle_position_range(_ nodeHandle: UnsafeMutableRawPointer?, _ dx: Double, _ dy: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particlePositionRange = CGVector(dx: dx, dy: dy)
}

@_cdecl("sk_emitter_node_get_particle_speed")
public func sk_emitter_node_get_particle_speed(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particleSpeed)
}

@_cdecl("sk_emitter_node_set_particle_speed")
public func sk_emitter_node_set_particle_speed(_ nodeHandle: UnsafeMutableRawPointer?, _ speed: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particleSpeed = CGFloat(speed)
}

@_cdecl("sk_emitter_node_get_particle_speed_range")
public func sk_emitter_node_get_particle_speed_range(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particleSpeedRange)
}

@_cdecl("sk_emitter_node_set_particle_speed_range")
public func sk_emitter_node_set_particle_speed_range(_ nodeHandle: UnsafeMutableRawPointer?, _ range: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particleSpeedRange = CGFloat(range)
}

@_cdecl("sk_emitter_node_get_emission_angle")
public func sk_emitter_node_get_emission_angle(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.emissionAngle)
}

@_cdecl("sk_emitter_node_set_emission_angle")
public func sk_emitter_node_set_emission_angle(_ nodeHandle: UnsafeMutableRawPointer?, _ angle: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.emissionAngle = CGFloat(angle)
}

@_cdecl("sk_emitter_node_get_emission_angle_range")
public func sk_emitter_node_get_emission_angle_range(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.emissionAngleRange)
}

@_cdecl("sk_emitter_node_set_emission_angle_range")
public func sk_emitter_node_set_emission_angle_range(_ nodeHandle: UnsafeMutableRawPointer?, _ range: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.emissionAngleRange = CGFloat(range)
}

@_cdecl("sk_emitter_node_get_x_acceleration")
public func sk_emitter_node_get_x_acceleration(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.xAcceleration)
}

@_cdecl("sk_emitter_node_set_x_acceleration")
public func sk_emitter_node_set_x_acceleration(_ nodeHandle: UnsafeMutableRawPointer?, _ acceleration: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.xAcceleration = CGFloat(acceleration)
}

@_cdecl("sk_emitter_node_get_y_acceleration")
public func sk_emitter_node_get_y_acceleration(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.yAcceleration)
}

@_cdecl("sk_emitter_node_set_y_acceleration")
public func sk_emitter_node_set_y_acceleration(_ nodeHandle: UnsafeMutableRawPointer?, _ acceleration: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.yAcceleration = CGFloat(acceleration)
}

@_cdecl("sk_emitter_node_get_particle_birth_rate")
public func sk_emitter_node_get_particle_birth_rate(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particleBirthRate)
}

@_cdecl("sk_emitter_node_set_particle_birth_rate")
public func sk_emitter_node_set_particle_birth_rate(_ nodeHandle: UnsafeMutableRawPointer?, _ birthRate: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particleBirthRate = CGFloat(birthRate)
}

@_cdecl("sk_emitter_node_get_num_particles_to_emit")
public func sk_emitter_node_get_num_particles_to_emit(_ nodeHandle: UnsafeMutableRawPointer?) -> Int {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return node.numParticlesToEmit
}

@_cdecl("sk_emitter_node_set_num_particles_to_emit")
public func sk_emitter_node_set_num_particles_to_emit(_ nodeHandle: UnsafeMutableRawPointer?, _ count: Int) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.numParticlesToEmit = count
}

@_cdecl("sk_emitter_node_get_particle_lifetime")
public func sk_emitter_node_get_particle_lifetime(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particleLifetime)
}

@_cdecl("sk_emitter_node_set_particle_lifetime")
public func sk_emitter_node_set_particle_lifetime(_ nodeHandle: UnsafeMutableRawPointer?, _ lifetime: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particleLifetime = CGFloat(lifetime)
}

@_cdecl("sk_emitter_node_get_particle_lifetime_range")
public func sk_emitter_node_get_particle_lifetime_range(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particleLifetimeRange)
}

@_cdecl("sk_emitter_node_set_particle_lifetime_range")
public func sk_emitter_node_set_particle_lifetime_range(_ nodeHandle: UnsafeMutableRawPointer?, _ range: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particleLifetimeRange = CGFloat(range)
}

@_cdecl("sk_emitter_node_get_particle_rotation")
public func sk_emitter_node_get_particle_rotation(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particleRotation)
}

@_cdecl("sk_emitter_node_set_particle_rotation")
public func sk_emitter_node_set_particle_rotation(_ nodeHandle: UnsafeMutableRawPointer?, _ rotation: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particleRotation = CGFloat(rotation)
}

@_cdecl("sk_emitter_node_get_particle_rotation_range")
public func sk_emitter_node_get_particle_rotation_range(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particleRotationRange)
}

@_cdecl("sk_emitter_node_set_particle_rotation_range")
public func sk_emitter_node_set_particle_rotation_range(_ nodeHandle: UnsafeMutableRawPointer?, _ range: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particleRotationRange = CGFloat(range)
}

@_cdecl("sk_emitter_node_get_particle_rotation_speed")
public func sk_emitter_node_get_particle_rotation_speed(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particleRotationSpeed)
}

@_cdecl("sk_emitter_node_set_particle_rotation_speed")
public func sk_emitter_node_set_particle_rotation_speed(_ nodeHandle: UnsafeMutableRawPointer?, _ speed: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particleRotationSpeed = CGFloat(speed)
}

@_cdecl("sk_emitter_node_get_particle_size_w")
public func sk_emitter_node_get_particle_size_w(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particleSize.width)
}

@_cdecl("sk_emitter_node_get_particle_size_h")
public func sk_emitter_node_get_particle_size_h(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particleSize.height)
}

@_cdecl("sk_emitter_node_set_particle_size")
public func sk_emitter_node_set_particle_size(_ nodeHandle: UnsafeMutableRawPointer?, _ width: Double, _ height: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particleSize = CGSize(width: width, height: height)
}

@_cdecl("sk_emitter_node_get_particle_scale")
public func sk_emitter_node_get_particle_scale(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particleScale)
}

@_cdecl("sk_emitter_node_set_particle_scale")
public func sk_emitter_node_set_particle_scale(_ nodeHandle: UnsafeMutableRawPointer?, _ scale: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particleScale = CGFloat(scale)
}

@_cdecl("sk_emitter_node_get_particle_scale_range")
public func sk_emitter_node_get_particle_scale_range(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particleScaleRange)
}

@_cdecl("sk_emitter_node_set_particle_scale_range")
public func sk_emitter_node_set_particle_scale_range(_ nodeHandle: UnsafeMutableRawPointer?, _ range: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particleScaleRange = CGFloat(range)
}

@_cdecl("sk_emitter_node_get_particle_scale_speed")
public func sk_emitter_node_get_particle_scale_speed(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particleScaleSpeed)
}

@_cdecl("sk_emitter_node_set_particle_scale_speed")
public func sk_emitter_node_set_particle_scale_speed(_ nodeHandle: UnsafeMutableRawPointer?, _ speed: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particleScaleSpeed = CGFloat(speed)
}

@_cdecl("sk_emitter_node_get_particle_alpha")
public func sk_emitter_node_get_particle_alpha(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particleAlpha)
}

@_cdecl("sk_emitter_node_set_particle_alpha")
public func sk_emitter_node_set_particle_alpha(_ nodeHandle: UnsafeMutableRawPointer?, _ alpha: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particleAlpha = CGFloat(alpha)
}

@_cdecl("sk_emitter_node_get_particle_alpha_range")
public func sk_emitter_node_get_particle_alpha_range(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particleAlphaRange)
}

@_cdecl("sk_emitter_node_set_particle_alpha_range")
public func sk_emitter_node_set_particle_alpha_range(_ nodeHandle: UnsafeMutableRawPointer?, _ range: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particleAlphaRange = CGFloat(range)
}

@_cdecl("sk_emitter_node_get_particle_alpha_speed")
public func sk_emitter_node_get_particle_alpha_speed(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.particleAlphaSpeed)
}

@_cdecl("sk_emitter_node_set_particle_alpha_speed")
public func sk_emitter_node_set_particle_alpha_speed(_ nodeHandle: UnsafeMutableRawPointer?, _ speed: Double) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.particleAlphaSpeed = CGFloat(speed)
}

@_cdecl("sk_emitter_node_get_field_bitmask")
public func sk_emitter_node_get_field_bitmask(_ nodeHandle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return 0 }
    return node.fieldBitMask
}

@_cdecl("sk_emitter_node_set_field_bitmask")
public func sk_emitter_node_set_field_bitmask(_ nodeHandle: UnsafeMutableRawPointer?, _ mask: UInt32) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    node.fieldBitMask = mask
}

@_cdecl("sk_emitter_node_set_target_node")
public func sk_emitter_node_set_target_node(_ nodeHandle: UnsafeMutableRawPointer?, _ targetNodeHandle: UnsafeMutableRawPointer?) {
    guard let node: SKEmitterNode = skBorrow(nodeHandle) else { return }
    let targetNode: SKNode? = skBorrow(targetNodeHandle)
    node.targetNode = targetNode
}
