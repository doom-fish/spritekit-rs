import SpriteKit

@_cdecl("sk_label_node_new_with_text")
public func sk_label_node_new_with_text(_ text: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    skRetain(SKLabelNode(text: text.map(String.init(cString:))))
}

@_cdecl("sk_label_node_new_with_font_named")
public func sk_label_node_new_with_font_named(_ fontName: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    skRetain(SKLabelNode(fontNamed: fontName.map(String.init(cString:))))
}

@_cdecl("sk_label_node_copy_font_name")
public func sk_label_node_copy_font_name(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return nil }
    return skDup(node.fontName)
}

@_cdecl("sk_label_node_set_font_name")
public func sk_label_node_set_font_name(_ nodeHandle: UnsafeMutableRawPointer?, _ fontName: UnsafePointer<CChar>?) {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return }
    node.fontName = fontName.map(String.init(cString:))
}

@_cdecl("sk_label_node_copy_text")
public func sk_label_node_copy_text(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return nil }
    return skDup(node.text)
}

@_cdecl("sk_label_node_set_text")
public func sk_label_node_set_text(_ nodeHandle: UnsafeMutableRawPointer?, _ text: UnsafePointer<CChar>?) {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return }
    node.text = text.map(String.init(cString:))
}

@_cdecl("sk_label_node_get_font_size")
public func sk_label_node_get_font_size(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.fontSize)
}

@_cdecl("sk_label_node_set_font_size")
public func sk_label_node_set_font_size(_ nodeHandle: UnsafeMutableRawPointer?, _ fontSize: Double) {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return }
    node.fontSize = CGFloat(fontSize)
}

@_cdecl("sk_label_node_get_vertical_alignment_mode")
public func sk_label_node_get_vertical_alignment_mode(_ nodeHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return 0 }
    return Int32(node.verticalAlignmentMode.rawValue)
}

@_cdecl("sk_label_node_set_vertical_alignment_mode")
public func sk_label_node_set_vertical_alignment_mode(_ nodeHandle: UnsafeMutableRawPointer?, _ mode: Int32) {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return }
    node.verticalAlignmentMode = SKLabelVerticalAlignmentMode(rawValue: Int(mode)) ?? .baseline
}

@_cdecl("sk_label_node_get_horizontal_alignment_mode")
public func sk_label_node_get_horizontal_alignment_mode(_ nodeHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return 0 }
    return Int32(node.horizontalAlignmentMode.rawValue)
}

@_cdecl("sk_label_node_set_horizontal_alignment_mode")
public func sk_label_node_set_horizontal_alignment_mode(_ nodeHandle: UnsafeMutableRawPointer?, _ mode: Int32) {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return }
    node.horizontalAlignmentMode = SKLabelHorizontalAlignmentMode(rawValue: Int(mode)) ?? .center
}

@_cdecl("sk_label_node_get_number_of_lines")
public func sk_label_node_get_number_of_lines(_ nodeHandle: UnsafeMutableRawPointer?) -> Int {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return 0 }
    return node.numberOfLines
}

@_cdecl("sk_label_node_set_number_of_lines")
public func sk_label_node_set_number_of_lines(_ nodeHandle: UnsafeMutableRawPointer?, _ lines: Int) {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return }
    node.numberOfLines = lines
}

@_cdecl("sk_label_node_get_preferred_max_layout_width")
public func sk_label_node_get_preferred_max_layout_width(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.preferredMaxLayoutWidth)
}

@_cdecl("sk_label_node_set_preferred_max_layout_width")
public func sk_label_node_set_preferred_max_layout_width(_ nodeHandle: UnsafeMutableRawPointer?, _ width: Double) {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return }
    node.preferredMaxLayoutWidth = CGFloat(width)
}

@_cdecl("sk_label_node_set_font_color")
public func sk_label_node_set_font_color(_ nodeHandle: UnsafeMutableRawPointer?, _ r: Float, _ g: Float, _ b: Float, _ a: Float) {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return }
    node.fontColor = skMakeColor(r: r, g: g, b: b, a: a)
}

@_cdecl("sk_label_node_get_color_blend_factor")
public func sk_label_node_get_color_blend_factor(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.colorBlendFactor)
}

@_cdecl("sk_label_node_set_color_blend_factor")
public func sk_label_node_set_color_blend_factor(_ nodeHandle: UnsafeMutableRawPointer?, _ factor: Double) {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return }
    node.colorBlendFactor = CGFloat(factor)
}

@_cdecl("sk_label_node_set_color")
public func sk_label_node_set_color(_ nodeHandle: UnsafeMutableRawPointer?, _ r: Float, _ g: Float, _ b: Float, _ a: Float) {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return }
    node.color = skMakeColor(r: r, g: g, b: b, a: a)
}

@_cdecl("sk_label_node_get_blend_mode")
public func sk_label_node_get_blend_mode(_ nodeHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return 0 }
    return Int32(node.blendMode.rawValue)
}

@_cdecl("sk_label_node_set_blend_mode")
public func sk_label_node_set_blend_mode(_ nodeHandle: UnsafeMutableRawPointer?, _ mode: Int32) {
    guard let node: SKLabelNode = skBorrow(nodeHandle) else { return }
    node.blendMode = skBlendMode(mode)
}
