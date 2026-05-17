import SpriteKit

private func skWarpPositions(from rawPositions: UnsafePointer<Float>?, count: Int) -> [vector_float2] {
    guard let rawPositions else { return [] }
    var positions: [vector_float2] = []
    positions.reserveCapacity(count)
    for index in 0..<count {
        let base = index * 2
        positions.append(vector_float2(rawPositions[base], rawPositions[base + 1]))
    }
    return positions
}

private func skBorrowWarpable(_ handle: UnsafeMutableRawPointer?) -> (AnyObject & SKWarpable)? {
    guard let handle else { return nil }
    return Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? (AnyObject & SKWarpable)
}

@_cdecl("sk_warp_geometry_grid")
public func sk_warp_geometry_grid() -> UnsafeMutableRawPointer? {
    skRetain(SKWarpGeometryGrid())
}

@_cdecl("sk_warp_geometry_grid_with_dimensions")
public func sk_warp_geometry_grid_with_dimensions(_ columns: Int, _ rows: Int) -> UnsafeMutableRawPointer? {
    skRetain(SKWarpGeometryGrid(__columns: columns, rows: rows, sourcePositions: nil, destPositions: nil))
}

@_cdecl("sk_warp_geometry_grid_with_positions")
public func sk_warp_geometry_grid_with_positions(
    _ columns: Int,
    _ rows: Int,
    _ sourcePositions: UnsafePointer<Float>?,
    _ destPositions: UnsafePointer<Float>?,
    _ count: Int
) -> UnsafeMutableRawPointer? {
    skRetain(SKWarpGeometryGrid(
        __columns: columns,
        rows: rows,
        sourcePositions: skWarpPositions(from: sourcePositions, count: count),
        destPositions: skWarpPositions(from: destPositions, count: count)
    ))
}

@_cdecl("sk_warp_geometry_grid_get_number_of_columns")
public func sk_warp_geometry_grid_get_number_of_columns(_ gridHandle: UnsafeMutableRawPointer?) -> Int {
    guard let grid: SKWarpGeometryGrid = skBorrow(gridHandle) else { return 0 }
    return grid.numberOfColumns
}

@_cdecl("sk_warp_geometry_grid_get_number_of_rows")
public func sk_warp_geometry_grid_get_number_of_rows(_ gridHandle: UnsafeMutableRawPointer?) -> Int {
    guard let grid: SKWarpGeometryGrid = skBorrow(gridHandle) else { return 0 }
    return grid.numberOfRows
}

@_cdecl("sk_warp_geometry_grid_get_vertex_count")
public func sk_warp_geometry_grid_get_vertex_count(_ gridHandle: UnsafeMutableRawPointer?) -> Int {
    guard let grid: SKWarpGeometryGrid = skBorrow(gridHandle) else { return 0 }
    return grid.vertexCount
}

@_cdecl("sk_warp_geometry_grid_get_source_position_x")
public func sk_warp_geometry_grid_get_source_position_x(_ gridHandle: UnsafeMutableRawPointer?, _ index: Int) -> Double {
    guard let grid: SKWarpGeometryGrid = skBorrow(gridHandle) else { return 0 }
    return Double(grid.sourcePosition(at: index).x)
}

@_cdecl("sk_warp_geometry_grid_get_source_position_y")
public func sk_warp_geometry_grid_get_source_position_y(_ gridHandle: UnsafeMutableRawPointer?, _ index: Int) -> Double {
    guard let grid: SKWarpGeometryGrid = skBorrow(gridHandle) else { return 0 }
    return Double(grid.sourcePosition(at: index).y)
}

@_cdecl("sk_warp_geometry_grid_get_dest_position_x")
public func sk_warp_geometry_grid_get_dest_position_x(_ gridHandle: UnsafeMutableRawPointer?, _ index: Int) -> Double {
    guard let grid: SKWarpGeometryGrid = skBorrow(gridHandle) else { return 0 }
    return Double(grid.destPosition(at: index).x)
}

@_cdecl("sk_warp_geometry_grid_get_dest_position_y")
public func sk_warp_geometry_grid_get_dest_position_y(_ gridHandle: UnsafeMutableRawPointer?, _ index: Int) -> Double {
    guard let grid: SKWarpGeometryGrid = skBorrow(gridHandle) else { return 0 }
    return Double(grid.destPosition(at: index).y)
}

@_cdecl("sk_warp_geometry_grid_replacing_source_positions")
public func sk_warp_geometry_grid_replacing_source_positions(_ gridHandle: UnsafeMutableRawPointer?, _ sourcePositions: UnsafePointer<Float>?, _ count: Int) -> UnsafeMutableRawPointer? {
    guard let grid: SKWarpGeometryGrid = skBorrow(gridHandle) else { return nil }
    return skRetain(grid.replacingBySourcePositions(positions: skWarpPositions(from: sourcePositions, count: count)))
}

@_cdecl("sk_warp_geometry_grid_replacing_dest_positions")
public func sk_warp_geometry_grid_replacing_dest_positions(_ gridHandle: UnsafeMutableRawPointer?, _ destPositions: UnsafePointer<Float>?, _ count: Int) -> UnsafeMutableRawPointer? {
    guard let grid: SKWarpGeometryGrid = skBorrow(gridHandle) else { return nil }
    return skRetain(grid.replacingByDestinationPositions(positions: skWarpPositions(from: destPositions, count: count)))
}

@_cdecl("sk_warpable_get_warp_geometry")
public func sk_warpable_get_warp_geometry(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node = skBorrowWarpable(nodeHandle),
          let warpGeometry = node.warpGeometry
    else { return nil }
    return skRetain(warpGeometry)
}

@_cdecl("sk_warpable_set_warp_geometry")
public func sk_warpable_set_warp_geometry(_ nodeHandle: UnsafeMutableRawPointer?, _ warpGeometryHandle: UnsafeMutableRawPointer?) {
    guard let node = skBorrowWarpable(nodeHandle) else { return }
    let warpGeometry: SKWarpGeometry? = skBorrow(warpGeometryHandle)
    node.warpGeometry = warpGeometry
}

@_cdecl("sk_warpable_get_subdivision_levels")
public func sk_warpable_get_subdivision_levels(_ nodeHandle: UnsafeMutableRawPointer?) -> Int {
    guard let node = skBorrowWarpable(nodeHandle) else { return 0 }
    return node.subdivisionLevels
}

@_cdecl("sk_warpable_set_subdivision_levels")
public func sk_warpable_set_subdivision_levels(_ nodeHandle: UnsafeMutableRawPointer?, _ levels: Int) {
    guard let node = skBorrowWarpable(nodeHandle) else { return }
    node.subdivisionLevels = levels
}
