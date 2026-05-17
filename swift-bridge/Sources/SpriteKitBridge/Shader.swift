import SpriteKit

@_cdecl("sk_shader_new")
public func sk_shader_new() -> UnsafeMutableRawPointer? {
    skRetain(SKShader())
}

@_cdecl("sk_shader_new_with_source")
public func sk_shader_new_with_source(_ source: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let source else { return nil }
    return skRetain(SKShader(source: String(cString: source)))
}

@_cdecl("sk_shader_copy_source")
public func sk_shader_copy_source(_ shaderHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let shader: SKShader = skBorrow(shaderHandle) else { return nil }
    return skDup(shader.source)
}

@_cdecl("sk_shader_set_source")
public func sk_shader_set_source(_ shaderHandle: UnsafeMutableRawPointer?, _ source: UnsafePointer<CChar>?) {
    guard let shader: SKShader = skBorrow(shaderHandle) else { return }
    shader.source = source.map(String.init(cString:))
}

@_cdecl("sk_shader_uniform_count")
public func sk_shader_uniform_count(_ shaderHandle: UnsafeMutableRawPointer?) -> Int {
    guard let shader: SKShader = skBorrow(shaderHandle) else { return 0 }
    return shader.uniforms.count
}

@_cdecl("sk_shader_add_float_uniform")
public func sk_shader_add_float_uniform(_ shaderHandle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?, _ value: Float) {
    guard let shader: SKShader = skBorrow(shaderHandle), let name else { return }
    shader.addUniform(SKUniform(name: String(cString: name), float: value))
}

@_cdecl("sk_shader_get_float_uniform")
public func sk_shader_get_float_uniform(_ shaderHandle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?, _ outValue: UnsafeMutablePointer<Float>?) -> Bool {
    guard let shader: SKShader = skBorrow(shaderHandle),
          let name,
          let outValue,
          let uniform = shader.uniformNamed(String(cString: name))
    else { return false }
    outValue.pointee = uniform.floatValue
    return true
}

@_cdecl("sk_shader_get_uniform_type")
public func sk_shader_get_uniform_type(_ shaderHandle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?, _ outValue: UnsafeMutablePointer<Int32>?) -> Bool {
    guard let shader: SKShader = skBorrow(shaderHandle),
          let name,
          let outValue,
          let uniform = shader.uniformNamed(String(cString: name))
    else { return false }
    outValue.pointee = Int32(uniform.uniformType.rawValue)
    return true
}

@_cdecl("sk_shader_remove_uniform_named")
public func sk_shader_remove_uniform_named(_ shaderHandle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) {
    guard let shader: SKShader = skBorrow(shaderHandle), let name else { return }
    shader.removeUniformNamed(String(cString: name))
}

@_cdecl("sk_shader_attribute_count")
public func sk_shader_attribute_count(_ shaderHandle: UnsafeMutableRawPointer?) -> Int {
    guard let shader: SKShader = skBorrow(shaderHandle) else { return 0 }
    return shader.attributes.count
}
