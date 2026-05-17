import SpriteKit

@_cdecl("sk_attribute_new")
public func sk_attribute_new(_ name: UnsafePointer<CChar>?, _ attributeType: Int32) -> UnsafeMutableRawPointer? {
    guard let name else { return nil }
    return skRetain(SKAttribute(name: String(cString: name), type: SKAttributeType(rawValue: Int(attributeType)) ?? .none))
}

@_cdecl("sk_attribute_copy_name")
public func sk_attribute_copy_name(_ attributeHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let attribute: SKAttribute = skBorrow(attributeHandle) else { return nil }
    return skDup(attribute.name)
}

@_cdecl("sk_attribute_get_type")
public func sk_attribute_get_type(_ attributeHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let attribute: SKAttribute = skBorrow(attributeHandle) else { return 0 }
    return Int32(attribute.type.rawValue)
}

@_cdecl("sk_attribute_value_new_with_float")
public func sk_attribute_value_new_with_float(_ value: Float) -> UnsafeMutableRawPointer? {
    skRetain(SKAttributeValue(float: value))
}

@_cdecl("sk_attribute_value_get_float")
public func sk_attribute_value_get_float(_ valueHandle: UnsafeMutableRawPointer?) -> Float {
    guard let value: SKAttributeValue = skBorrow(valueHandle) else { return 0 }
    return value.floatValue
}

@_cdecl("sk_attribute_value_set_float")
public func sk_attribute_value_set_float(_ valueHandle: UnsafeMutableRawPointer?, _ newValue: Float) {
    guard let value: SKAttributeValue = skBorrow(valueHandle) else { return }
    value.floatValue = newValue
}
