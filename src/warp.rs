use apple_cf::cg::CGPoint;

use crate::effect_node::EffectNode;
use crate::ffi;
use crate::private::handle_type;
use crate::sprite_node::SpriteNode;

handle_type!(WarpGeometry);
handle_type!(WarpGeometryGrid);

pub trait AsWarpGeometry {
    #[doc(hidden)]
    fn as_warp_geometry_ptr(&self) -> *mut core::ffi::c_void;
}

impl AsWarpGeometry for WarpGeometry {
    fn as_warp_geometry_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl AsWarpGeometry for WarpGeometryGrid {
    fn as_warp_geometry_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

#[allow(clippy::cast_possible_truncation)]
fn pack_positions(points: &[CGPoint]) -> Vec<f32> {
    let mut packed = Vec::with_capacity(points.len() * 2);
    for point in points {
        packed.push(point.x as f32);
        packed.push(point.y as f32);
    }
    packed
}

impl WarpGeometryGrid {
    #[must_use]
    pub fn grid() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_warp_geometry_grid()) }
    }

    #[must_use]
    pub fn with_dimensions(columns: usize, rows: usize) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_warp_geometry_grid_with_dimensions(columns, rows)) }
    }

    #[must_use]
    pub fn with_positions(columns: usize, rows: usize, source_positions: &[CGPoint], dest_positions: &[CGPoint]) -> Option<Self> {
        let expected = (columns + 1) * (rows + 1);
        if source_positions.len() != expected || dest_positions.len() != expected {
            return None;
        }
        let source = pack_positions(source_positions);
        let dest = pack_positions(dest_positions);
        unsafe {
            Self::from_raw(ffi::sk_warp_geometry_grid_with_positions(
                columns,
                rows,
                source.as_ptr(),
                dest.as_ptr(),
                expected,
            ))
        }
    }

    #[must_use]
    pub fn number_of_columns(&self) -> usize {
        unsafe { ffi::sk_warp_geometry_grid_get_number_of_columns(self.ptr) }
    }

    #[must_use]
    pub fn number_of_rows(&self) -> usize {
        unsafe { ffi::sk_warp_geometry_grid_get_number_of_rows(self.ptr) }
    }

    #[must_use]
    pub fn vertex_count(&self) -> usize {
        unsafe { ffi::sk_warp_geometry_grid_get_vertex_count(self.ptr) }
    }

    #[must_use]
    pub fn source_position_at(&self, index: usize) -> CGPoint {
        CGPoint::new(
            unsafe { ffi::sk_warp_geometry_grid_get_source_position_x(self.ptr, index) },
            unsafe { ffi::sk_warp_geometry_grid_get_source_position_y(self.ptr, index) },
        )
    }

    #[must_use]
    pub fn dest_position_at(&self, index: usize) -> CGPoint {
        CGPoint::new(
            unsafe { ffi::sk_warp_geometry_grid_get_dest_position_x(self.ptr, index) },
            unsafe { ffi::sk_warp_geometry_grid_get_dest_position_y(self.ptr, index) },
        )
    }

    #[must_use]
    pub fn replacing_source_positions(&self, source_positions: &[CGPoint]) -> Option<Self> {
        if source_positions.len() != self.vertex_count() {
            return None;
        }
        let source = pack_positions(source_positions);
        unsafe { Self::from_raw(ffi::sk_warp_geometry_grid_replacing_source_positions(self.ptr, source.as_ptr(), source_positions.len())) }
    }

    #[must_use]
    pub fn replacing_dest_positions(&self, dest_positions: &[CGPoint]) -> Option<Self> {
        if dest_positions.len() != self.vertex_count() {
            return None;
        }
        let dest = pack_positions(dest_positions);
        unsafe { Self::from_raw(ffi::sk_warp_geometry_grid_replacing_dest_positions(self.ptr, dest.as_ptr(), dest_positions.len())) }
    }
}

pub trait WarpableNode {
    #[doc(hidden)]
    fn as_warpable_ptr(&self) -> *mut core::ffi::c_void;

    #[must_use]
    fn warp_geometry(&self) -> Option<WarpGeometry> {
        unsafe { WarpGeometry::from_raw(ffi::sk_warpable_get_warp_geometry(self.as_warpable_ptr())) }
    }

    fn set_warp_geometry<W: AsWarpGeometry>(&self, warp: Option<&W>) {
        unsafe {
            ffi::sk_warpable_set_warp_geometry(
                self.as_warpable_ptr(),
                warp.map_or(core::ptr::null_mut(), AsWarpGeometry::as_warp_geometry_ptr),
            );
        };
    }

    #[must_use]
    fn subdivision_levels(&self) -> isize {
        unsafe { ffi::sk_warpable_get_subdivision_levels(self.as_warpable_ptr()) }
    }

    fn set_subdivision_levels(&self, levels: isize) {
        unsafe { ffi::sk_warpable_set_subdivision_levels(self.as_warpable_ptr(), levels) };
    }
}

impl WarpableNode for SpriteNode {
    fn as_warpable_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl WarpableNode for EffectNode {
    fn as_warpable_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}
