import SpriteKit

@_cdecl("sk_region_infinite")
public func sk_region_infinite() -> UnsafeMutableRawPointer? {
    skRetain(SKRegion.infinite())
}

@_cdecl("sk_region_new_with_radius")
public func sk_region_new_with_radius(_ radius: Float) -> UnsafeMutableRawPointer? {
    skRetain(SKRegion(radius: radius))
}

@_cdecl("sk_region_new_with_size")
public func sk_region_new_with_size(_ width: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKRegion(size: CGSize(width: width, height: height)))
}

@_cdecl("sk_region_inverse")
public func sk_region_inverse(_ regionHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let region: SKRegion = skBorrow(regionHandle) else { return nil }
    return skRetain(region.inverse())
}

@_cdecl("sk_region_union")
public func sk_region_union(_ regionHandle: UnsafeMutableRawPointer?, _ otherHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let region: SKRegion = skBorrow(regionHandle),
          let other: SKRegion = skBorrow(otherHandle)
    else { return nil }
    return skRetain(region.byUnion(with: other))
}

@_cdecl("sk_region_difference")
public func sk_region_difference(_ regionHandle: UnsafeMutableRawPointer?, _ otherHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let region: SKRegion = skBorrow(regionHandle),
          let other: SKRegion = skBorrow(otherHandle)
    else { return nil }
    return skRetain(region.byDifference(from: other))
}

@_cdecl("sk_region_intersection")
public func sk_region_intersection(_ regionHandle: UnsafeMutableRawPointer?, _ otherHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let region: SKRegion = skBorrow(regionHandle),
          let other: SKRegion = skBorrow(otherHandle)
    else { return nil }
    return skRetain(region.byIntersection(with: other))
}

@_cdecl("sk_region_contains_point")
public func sk_region_contains_point(_ regionHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) -> Bool {
    guard let region: SKRegion = skBorrow(regionHandle) else { return false }
    return region.contains(CGPoint(x: x, y: y))
}
