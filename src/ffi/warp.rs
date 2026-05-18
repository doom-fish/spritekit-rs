use core::ffi::c_void;

extern "C" {
    /// Wraps `SKWarpGeometry`, `SKWarpGeometryGrid`, and `SKWarpable`.
    pub fn sk_warp_geometry_grid() -> *mut c_void;
    /// Wraps `SKWarpGeometry`, `SKWarpGeometryGrid`, and `SKWarpable`.
    pub fn sk_warp_geometry_grid_with_dimensions(columns: usize, rows: usize) -> *mut c_void;
    /// Wraps `SKWarpGeometry`, `SKWarpGeometryGrid`, and `SKWarpable`.
    pub fn sk_warp_geometry_grid_with_positions(
        columns: usize,
        rows: usize,
        source_positions: *const f32,
        dest_positions: *const f32,
        count: usize,
    ) -> *mut c_void;
    /// Wraps `SKWarpGeometry`, `SKWarpGeometryGrid`, and `SKWarpable`.
    pub fn sk_warp_geometry_grid_get_number_of_columns(grid: *mut c_void) -> usize;
    /// Wraps `SKWarpGeometry`, `SKWarpGeometryGrid`, and `SKWarpable`.
    pub fn sk_warp_geometry_grid_get_number_of_rows(grid: *mut c_void) -> usize;
    /// Returns a property exposed by `SKWarpGeometry`, `SKWarpGeometryGrid`, and `SKWarpable`.
    pub fn sk_warp_geometry_grid_get_vertex_count(grid: *mut c_void) -> usize;
    /// Wraps `SKWarpGeometry`, `SKWarpGeometryGrid`, and `SKWarpable`.
    pub fn sk_warp_geometry_grid_get_source_position_x(grid: *mut c_void, index: usize) -> f64;
    /// Wraps `SKWarpGeometry`, `SKWarpGeometryGrid`, and `SKWarpable`.
    pub fn sk_warp_geometry_grid_get_source_position_y(grid: *mut c_void, index: usize) -> f64;
    /// Wraps `SKWarpGeometry`, `SKWarpGeometryGrid`, and `SKWarpable`.
    pub fn sk_warp_geometry_grid_get_dest_position_x(grid: *mut c_void, index: usize) -> f64;
    /// Wraps `SKWarpGeometry`, `SKWarpGeometryGrid`, and `SKWarpable`.
    pub fn sk_warp_geometry_grid_get_dest_position_y(grid: *mut c_void, index: usize) -> f64;
    /// Wraps `SKWarpGeometry`, `SKWarpGeometryGrid`, and `SKWarpable`.
    pub fn sk_warp_geometry_grid_replacing_source_positions(
        grid: *mut c_void,
        source_positions: *const f32,
        count: usize,
    ) -> *mut c_void;
    /// Wraps `SKWarpGeometry`, `SKWarpGeometryGrid`, and `SKWarpable`.
    pub fn sk_warp_geometry_grid_replacing_dest_positions(
        grid: *mut c_void,
        dest_positions: *const f32,
        count: usize,
    ) -> *mut c_void;

    /// Wraps `SKWarpGeometry`, `SKWarpGeometryGrid`, and `SKWarpable`.
    pub fn sk_warpable_get_warp_geometry(node: *mut c_void) -> *mut c_void;
    /// Wraps `SKWarpGeometry`, `SKWarpGeometryGrid`, and `SKWarpable`.
    pub fn sk_warpable_set_warp_geometry(node: *mut c_void, warp_geometry: *mut c_void);
    /// Wraps `SKWarpGeometry`, `SKWarpGeometryGrid`, and `SKWarpable`.
    pub fn sk_warpable_get_subdivision_levels(node: *mut c_void) -> isize;
    /// Wraps `SKWarpGeometry`, `SKWarpGeometryGrid`, and `SKWarpable`.
    pub fn sk_warpable_set_subdivision_levels(node: *mut c_void, levels: isize);
}
