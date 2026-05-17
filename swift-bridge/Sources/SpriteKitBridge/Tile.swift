import SpriteKit

private func skTileDefinitions(from rawDefinitions: UnsafeMutableRawPointer?, count: Int) -> [SKTileDefinition] {
    skHandles(from: rawDefinitions, count: count)
}

private func skTileGroupRules(from rawRules: UnsafeMutableRawPointer?, count: Int) -> [SKTileGroupRule] {
    skHandles(from: rawRules, count: count)
}

private func skTileGroups(from rawGroups: UnsafeMutableRawPointer?, count: Int) -> [SKTileGroup] {
    skHandles(from: rawGroups, count: count)
}

private func skTileDefinitionRotation(_ rawValue: UInt64) -> SKTileDefinitionRotation {
    SKTileDefinitionRotation(rawValue: UInt(rawValue)) ?? .rotation0
}

private func skTileSetType(_ rawValue: UInt64) -> SKTileSetType {
    SKTileSetType(rawValue: UInt(rawValue)) ?? .grid
}

@_cdecl("sk_tile_definition_new_with_texture")
public func sk_tile_definition_new_with_texture(_ textureHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let texture: SKTexture = skBorrow(textureHandle) else { return nil }
    return skRetain(SKTileDefinition(texture: texture))
}

@_cdecl("sk_tile_definition_new_with_texture_size")
public func sk_tile_definition_new_with_texture_size(_ textureHandle: UnsafeMutableRawPointer?, _ width: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    guard let texture: SKTexture = skBorrow(textureHandle) else { return nil }
    return skRetain(SKTileDefinition(texture: texture, size: CGSize(width: width, height: height)))
}

@_cdecl("sk_tile_definition_copy_name")
public func sk_tile_definition_copy_name(_ definitionHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let definition: SKTileDefinition = skBorrow(definitionHandle) else { return nil }
    return skDup(definition.name)
}

@_cdecl("sk_tile_definition_set_name")
public func sk_tile_definition_set_name(_ definitionHandle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) {
    guard let definition: SKTileDefinition = skBorrow(definitionHandle), let name else { return }
    definition.name = String(cString: name)
}

@_cdecl("sk_tile_definition_get_size_w")
public func sk_tile_definition_get_size_w(_ definitionHandle: UnsafeMutableRawPointer?) -> Double {
    guard let definition: SKTileDefinition = skBorrow(definitionHandle) else { return 0 }
    return Double(definition.size.width)
}

@_cdecl("sk_tile_definition_get_size_h")
public func sk_tile_definition_get_size_h(_ definitionHandle: UnsafeMutableRawPointer?) -> Double {
    guard let definition: SKTileDefinition = skBorrow(definitionHandle) else { return 0 }
    return Double(definition.size.height)
}

@_cdecl("sk_tile_definition_set_size")
public func sk_tile_definition_set_size(_ definitionHandle: UnsafeMutableRawPointer?, _ width: Double, _ height: Double) {
    guard let definition: SKTileDefinition = skBorrow(definitionHandle) else { return }
    definition.size = CGSize(width: width, height: height)
}

@_cdecl("sk_tile_definition_get_placement_weight")
public func sk_tile_definition_get_placement_weight(_ definitionHandle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let definition: SKTileDefinition = skBorrow(definitionHandle) else { return 0 }
    return UInt64(definition.placementWeight)
}

@_cdecl("sk_tile_definition_set_placement_weight")
public func sk_tile_definition_set_placement_weight(_ definitionHandle: UnsafeMutableRawPointer?, _ weight: UInt64) {
    guard let definition: SKTileDefinition = skBorrow(definitionHandle) else { return }
    definition.placementWeight = Int(weight)
}

@_cdecl("sk_tile_definition_get_rotation")
public func sk_tile_definition_get_rotation(_ definitionHandle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let definition: SKTileDefinition = skBorrow(definitionHandle) else { return 0 }
    return UInt64(definition.rotation.rawValue)
}

@_cdecl("sk_tile_definition_set_rotation")
public func sk_tile_definition_set_rotation(_ definitionHandle: UnsafeMutableRawPointer?, _ rotation: UInt64) {
    guard let definition: SKTileDefinition = skBorrow(definitionHandle) else { return }
    definition.rotation = skTileDefinitionRotation(rotation)
}

@_cdecl("sk_tile_definition_get_flip_vertically")
public func sk_tile_definition_get_flip_vertically(_ definitionHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let definition: SKTileDefinition = skBorrow(definitionHandle) else { return false }
    return definition.flipVertically
}

@_cdecl("sk_tile_definition_set_flip_vertically")
public func sk_tile_definition_set_flip_vertically(_ definitionHandle: UnsafeMutableRawPointer?, _ flip: Bool) {
    guard let definition: SKTileDefinition = skBorrow(definitionHandle) else { return }
    definition.flipVertically = flip
}

@_cdecl("sk_tile_definition_get_flip_horizontally")
public func sk_tile_definition_get_flip_horizontally(_ definitionHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let definition: SKTileDefinition = skBorrow(definitionHandle) else { return false }
    return definition.flipHorizontally
}

@_cdecl("sk_tile_definition_set_flip_horizontally")
public func sk_tile_definition_set_flip_horizontally(_ definitionHandle: UnsafeMutableRawPointer?, _ flip: Bool) {
    guard let definition: SKTileDefinition = skBorrow(definitionHandle) else { return }
    definition.flipHorizontally = flip
}

@_cdecl("sk_tile_group_rule_new")
public func sk_tile_group_rule_new(_ adjacency: UInt64, _ rawDefinitions: UnsafeMutableRawPointer?, _ count: Int) -> UnsafeMutableRawPointer? {
    skRetain(SKTileGroupRule(adjacency: SKTileAdjacencyMask(rawValue: UInt(adjacency)), tileDefinitions: skTileDefinitions(from: rawDefinitions, count: count)))
}

@_cdecl("sk_tile_group_rule_get_adjacency")
public func sk_tile_group_rule_get_adjacency(_ ruleHandle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let rule: SKTileGroupRule = skBorrow(ruleHandle) else { return 0 }
    return UInt64(rule.adjacency.rawValue)
}

@_cdecl("sk_tile_group_rule_set_adjacency")
public func sk_tile_group_rule_set_adjacency(_ ruleHandle: UnsafeMutableRawPointer?, _ adjacency: UInt64) {
    guard let rule: SKTileGroupRule = skBorrow(ruleHandle) else { return }
    rule.adjacency = SKTileAdjacencyMask(rawValue: UInt(adjacency))
}

@_cdecl("sk_tile_group_rule_copy_name")
public func sk_tile_group_rule_copy_name(_ ruleHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let rule: SKTileGroupRule = skBorrow(ruleHandle) else { return nil }
    return skDup(rule.name)
}

@_cdecl("sk_tile_group_rule_set_name")
public func sk_tile_group_rule_set_name(_ ruleHandle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) {
    guard let rule: SKTileGroupRule = skBorrow(ruleHandle), let name else { return }
    rule.name = String(cString: name)
}

@_cdecl("sk_tile_group_new_with_tile_definition")
public func sk_tile_group_new_with_tile_definition(_ definitionHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let definition: SKTileDefinition = skBorrow(definitionHandle) else { return nil }
    return skRetain(SKTileGroup(tileDefinition: definition))
}

@_cdecl("sk_tile_group_new_with_rules")
public func sk_tile_group_new_with_rules(_ rawRules: UnsafeMutableRawPointer?, _ count: Int) -> UnsafeMutableRawPointer? {
    skRetain(SKTileGroup(rules: skTileGroupRules(from: rawRules, count: count)))
}

@_cdecl("sk_tile_group_empty")
public func sk_tile_group_empty() -> UnsafeMutableRawPointer? {
    skRetain(SKTileGroup.empty())
}

@_cdecl("sk_tile_group_copy_name")
public func sk_tile_group_copy_name(_ groupHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let group: SKTileGroup = skBorrow(groupHandle) else { return nil }
    return skDup(group.name)
}

@_cdecl("sk_tile_group_set_name")
public func sk_tile_group_set_name(_ groupHandle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) {
    guard let group: SKTileGroup = skBorrow(groupHandle), let name else { return }
    group.name = String(cString: name)
}

@_cdecl("sk_tile_set_new")
public func sk_tile_set_new(_ rawGroups: UnsafeMutableRawPointer?, _ count: Int) -> UnsafeMutableRawPointer? {
    skRetain(SKTileSet(tileGroups: skTileGroups(from: rawGroups, count: count)))
}

@_cdecl("sk_tile_set_new_with_type")
public func sk_tile_set_new_with_type(_ rawGroups: UnsafeMutableRawPointer?, _ count: Int, _ tileSetType: UInt64) -> UnsafeMutableRawPointer? {
    skRetain(SKTileSet(tileGroups: skTileGroups(from: rawGroups, count: count), tileSetType: skTileSetType(tileSetType)))
}

@_cdecl("sk_tile_set_copy_name")
public func sk_tile_set_copy_name(_ tileSetHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let tileSet: SKTileSet = skBorrow(tileSetHandle) else { return nil }
    return skDup(tileSet.name)
}

@_cdecl("sk_tile_set_set_name")
public func sk_tile_set_set_name(_ tileSetHandle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) {
    guard let tileSet: SKTileSet = skBorrow(tileSetHandle), let name else { return }
    tileSet.name = String(cString: name)
}

@_cdecl("sk_tile_set_get_type")
public func sk_tile_set_get_type(_ tileSetHandle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let tileSet: SKTileSet = skBorrow(tileSetHandle) else { return 0 }
    return UInt64(tileSet.type.rawValue)
}

@_cdecl("sk_tile_set_set_type")
public func sk_tile_set_set_type(_ tileSetHandle: UnsafeMutableRawPointer?, _ tileSetType: UInt64) {
    guard let tileSet: SKTileSet = skBorrow(tileSetHandle) else { return }
    tileSet.type = skTileSetType(tileSetType)
}

@_cdecl("sk_tile_set_get_default_tile_group")
public func sk_tile_set_get_default_tile_group(_ tileSetHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let tileSet: SKTileSet = skBorrow(tileSetHandle),
          let tileGroup = tileSet.defaultTileGroup
    else { return nil }
    return skRetain(tileGroup)
}

@_cdecl("sk_tile_set_set_default_tile_group")
public func sk_tile_set_set_default_tile_group(_ tileSetHandle: UnsafeMutableRawPointer?, _ tileGroupHandle: UnsafeMutableRawPointer?) {
    guard let tileSet: SKTileSet = skBorrow(tileSetHandle) else { return }
    let tileGroup: SKTileGroup? = skBorrow(tileGroupHandle)
    tileSet.defaultTileGroup = tileGroup
}

@_cdecl("sk_tile_set_get_default_tile_size_w")
public func sk_tile_set_get_default_tile_size_w(_ tileSetHandle: UnsafeMutableRawPointer?) -> Double {
    guard let tileSet: SKTileSet = skBorrow(tileSetHandle) else { return 0 }
    return Double(tileSet.defaultTileSize.width)
}

@_cdecl("sk_tile_set_get_default_tile_size_h")
public func sk_tile_set_get_default_tile_size_h(_ tileSetHandle: UnsafeMutableRawPointer?) -> Double {
    guard let tileSet: SKTileSet = skBorrow(tileSetHandle) else { return 0 }
    return Double(tileSet.defaultTileSize.height)
}

@_cdecl("sk_tile_set_set_default_tile_size")
public func sk_tile_set_set_default_tile_size(_ tileSetHandle: UnsafeMutableRawPointer?, _ width: Double, _ height: Double) {
    guard let tileSet: SKTileSet = skBorrow(tileSetHandle) else { return }
    tileSet.defaultTileSize = CGSize(width: width, height: height)
}

@_cdecl("sk_tile_map_node_new")
public func sk_tile_map_node_new(_ tileSetHandle: UnsafeMutableRawPointer?, _ columns: Int, _ rows: Int, _ tileWidth: Double, _ tileHeight: Double) -> UnsafeMutableRawPointer? {
    guard let tileSet: SKTileSet = skBorrow(tileSetHandle) else { return nil }
    return skRetain(SKTileMapNode(tileSet: tileSet, columns: columns, rows: rows, tileSize: CGSize(width: tileWidth, height: tileHeight)))
}

@_cdecl("sk_tile_map_node_new_with_fill")
public func sk_tile_map_node_new_with_fill(_ tileSetHandle: UnsafeMutableRawPointer?, _ columns: Int, _ rows: Int, _ tileWidth: Double, _ tileHeight: Double, _ tileGroupHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let tileSet: SKTileSet = skBorrow(tileSetHandle),
          let tileGroup: SKTileGroup = skBorrow(tileGroupHandle)
    else { return nil }
    return skRetain(SKTileMapNode(tileSet: tileSet, columns: columns, rows: rows, tileSize: CGSize(width: tileWidth, height: tileHeight), fillWith: tileGroup))
}

@_cdecl("sk_tile_map_node_get_number_of_columns")
public func sk_tile_map_node_get_number_of_columns(_ nodeHandle: UnsafeMutableRawPointer?) -> Int {
    guard let node: SKTileMapNode = skBorrow(nodeHandle) else { return 0 }
    return node.numberOfColumns
}

@_cdecl("sk_tile_map_node_get_number_of_rows")
public func sk_tile_map_node_get_number_of_rows(_ nodeHandle: UnsafeMutableRawPointer?) -> Int {
    guard let node: SKTileMapNode = skBorrow(nodeHandle) else { return 0 }
    return node.numberOfRows
}

@_cdecl("sk_tile_map_node_get_tile_size_w")
public func sk_tile_map_node_get_tile_size_w(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKTileMapNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.tileSize.width)
}

@_cdecl("sk_tile_map_node_get_tile_size_h")
public func sk_tile_map_node_get_tile_size_h(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKTileMapNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.tileSize.height)
}

@_cdecl("sk_tile_map_node_set_tile_size")
public func sk_tile_map_node_set_tile_size(_ nodeHandle: UnsafeMutableRawPointer?, _ width: Double, _ height: Double) {
    guard let node: SKTileMapNode = skBorrow(nodeHandle) else { return }
    node.tileSize = CGSize(width: width, height: height)
}

@_cdecl("sk_tile_map_node_get_map_size_w")
public func sk_tile_map_node_get_map_size_w(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKTileMapNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.mapSize.width)
}

@_cdecl("sk_tile_map_node_get_map_size_h")
public func sk_tile_map_node_get_map_size_h(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKTileMapNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.mapSize.height)
}

@_cdecl("sk_tile_map_node_get_tile_set")
public func sk_tile_map_node_get_tile_set(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SKTileMapNode = skBorrow(nodeHandle) else { return nil }
    return skRetain(node.tileSet)
}

@_cdecl("sk_tile_map_node_set_tile_set")
public func sk_tile_map_node_set_tile_set(_ nodeHandle: UnsafeMutableRawPointer?, _ tileSetHandle: UnsafeMutableRawPointer?) {
    guard let node: SKTileMapNode = skBorrow(nodeHandle),
          let tileSet: SKTileSet = skBorrow(tileSetHandle)
    else { return }
    node.tileSet = tileSet
}

@_cdecl("sk_tile_map_node_fill_with_tile_group")
public func sk_tile_map_node_fill_with_tile_group(_ nodeHandle: UnsafeMutableRawPointer?, _ tileGroupHandle: UnsafeMutableRawPointer?) {
    guard let node: SKTileMapNode = skBorrow(nodeHandle) else { return }
    let tileGroup: SKTileGroup? = skBorrow(tileGroupHandle)
    node.fill(with: tileGroup)
}

@_cdecl("sk_tile_map_node_get_tile_group")
public func sk_tile_map_node_get_tile_group(_ nodeHandle: UnsafeMutableRawPointer?, _ column: Int, _ row: Int) -> UnsafeMutableRawPointer? {
    guard let node: SKTileMapNode = skBorrow(nodeHandle),
          let tileGroup = node.tileGroup(atColumn: column, row: row)
    else { return nil }
    return skRetain(tileGroup)
}

@_cdecl("sk_tile_map_node_set_tile_group")
public func sk_tile_map_node_set_tile_group(_ nodeHandle: UnsafeMutableRawPointer?, _ tileGroupHandle: UnsafeMutableRawPointer?, _ column: Int, _ row: Int) {
    guard let node: SKTileMapNode = skBorrow(nodeHandle) else { return }
    let tileGroup: SKTileGroup? = skBorrow(tileGroupHandle)
    node.setTileGroup(tileGroup, forColumn: column, row: row)
}

@_cdecl("sk_tile_map_node_get_tile_definition")
public func sk_tile_map_node_get_tile_definition(_ nodeHandle: UnsafeMutableRawPointer?, _ column: Int, _ row: Int) -> UnsafeMutableRawPointer? {
    guard let node: SKTileMapNode = skBorrow(nodeHandle),
          let tileDefinition = node.tileDefinition(atColumn: column, row: row)
    else { return nil }
    return skRetain(tileDefinition)
}

@_cdecl("sk_tile_map_node_center_of_tile_x")
public func sk_tile_map_node_center_of_tile_x(_ nodeHandle: UnsafeMutableRawPointer?, _ column: Int, _ row: Int) -> Double {
    guard let node: SKTileMapNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.centerOfTile(atColumn: column, row: row).x)
}

@_cdecl("sk_tile_map_node_center_of_tile_y")
public func sk_tile_map_node_center_of_tile_y(_ nodeHandle: UnsafeMutableRawPointer?, _ column: Int, _ row: Int) -> Double {
    guard let node: SKTileMapNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.centerOfTile(atColumn: column, row: row).y)
}

@_cdecl("sk_tile_map_node_tile_column_index_from_position")
public func sk_tile_map_node_tile_column_index_from_position(_ nodeHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) -> Int {
    guard let node: SKTileMapNode = skBorrow(nodeHandle) else { return .max }
    return node.tileColumnIndex(fromPosition: CGPoint(x: x, y: y))
}

@_cdecl("sk_tile_map_node_tile_row_index_from_position")
public func sk_tile_map_node_tile_row_index_from_position(_ nodeHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) -> Int {
    guard let node: SKTileMapNode = skBorrow(nodeHandle) else { return .max }
    return node.tileRowIndex(fromPosition: CGPoint(x: x, y: y))
}
