use core::ffi::c_void;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

use apple_cf::cg::{CGPoint, CGSize};

use crate::ffi;
use crate::node::AsNode;
use crate::private::{cstring_from_str, handle_type};
use crate::texture::Texture;

handle_type!(TileDefinition);
handle_type!(TileGroupRule);
handle_type!(TileGroup);
handle_type!(TileSet);
handle_type!(TileMapNode);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u64)]
pub enum TileDefinitionRotation {
    #[default]
    Rotation0 = 0,
    Rotation90 = 1,
    Rotation180 = 2,
    Rotation270 = 3,
}

impl TileDefinitionRotation {
    #[must_use]
    pub const fn from_raw(value: u64) -> Self {
        match value {
            1 => Self::Rotation90,
            2 => Self::Rotation180,
            3 => Self::Rotation270,
            _ => Self::Rotation0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u64)]
pub enum TileSetType {
    #[default]
    Grid = 0,
    Isometric = 1,
    HexagonalFlat = 2,
    HexagonalPointy = 3,
}

impl TileSetType {
    #[must_use]
    pub const fn from_raw(value: u64) -> Self {
        match value {
            1 => Self::Isometric,
            2 => Self::HexagonalFlat,
            3 => Self::HexagonalPointy,
            _ => Self::Grid,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TileAdjacencyMask(pub u64);

impl TileAdjacencyMask {
    pub const NONE: Self = Self(0);
    pub const UP: Self = Self(1 << 0);
    pub const UPPER_RIGHT: Self = Self(1 << 1);
    pub const RIGHT: Self = Self(1 << 2);
    pub const LOWER_RIGHT: Self = Self(1 << 3);
    pub const DOWN: Self = Self(1 << 4);
    pub const LOWER_LEFT: Self = Self(1 << 5);
    pub const LEFT: Self = Self(1 << 6);
    pub const UPPER_LEFT: Self = Self(1 << 7);
    pub const ALL: Self = Self(
        Self::UP.0
            | Self::UPPER_RIGHT.0
            | Self::RIGHT.0
            | Self::LOWER_RIGHT.0
            | Self::DOWN.0
            | Self::LOWER_LEFT.0
            | Self::LEFT.0
            | Self::UPPER_LEFT.0,
    );

    pub const HEX_FLAT_UP: Self = Self(1 << 0);
    pub const HEX_FLAT_UPPER_RIGHT: Self = Self(1 << 1);
    pub const HEX_FLAT_LOWER_RIGHT: Self = Self(1 << 2);
    pub const HEX_FLAT_DOWN: Self = Self(1 << 3);
    pub const HEX_FLAT_LOWER_LEFT: Self = Self(1 << 4);
    pub const HEX_FLAT_UPPER_LEFT: Self = Self(1 << 5);
    pub const HEX_FLAT_ALL: Self = Self(
        Self::HEX_FLAT_UP.0
            | Self::HEX_FLAT_UPPER_RIGHT.0
            | Self::HEX_FLAT_LOWER_RIGHT.0
            | Self::HEX_FLAT_DOWN.0
            | Self::HEX_FLAT_LOWER_LEFT.0
            | Self::HEX_FLAT_UPPER_LEFT.0,
    );

    pub const HEX_POINTY_UPPER_LEFT: Self = Self(1 << 0);
    pub const HEX_POINTY_UPPER_RIGHT: Self = Self(1 << 1);
    pub const HEX_POINTY_RIGHT: Self = Self(1 << 2);
    pub const HEX_POINTY_LOWER_RIGHT: Self = Self(1 << 3);
    pub const HEX_POINTY_LOWER_LEFT: Self = Self(1 << 4);
    pub const HEX_POINTY_LEFT: Self = Self(1 << 5);
    pub const HEX_POINTY_ALL: Self = Self(
        Self::HEX_POINTY_UPPER_LEFT.0
            | Self::HEX_POINTY_UPPER_RIGHT.0
            | Self::HEX_POINTY_RIGHT.0
            | Self::HEX_POINTY_LOWER_RIGHT.0
            | Self::HEX_POINTY_LOWER_LEFT.0
            | Self::HEX_POINTY_LEFT.0,
    );

    pub const UP_EDGE: Self = Self(
        Self::RIGHT.0 | Self::LOWER_RIGHT.0 | Self::DOWN.0 | Self::LOWER_LEFT.0 | Self::LEFT.0,
    );
    pub const UPPER_RIGHT_EDGE: Self = Self(Self::DOWN.0 | Self::LOWER_LEFT.0 | Self::LEFT.0);
    pub const RIGHT_EDGE: Self =
        Self(Self::DOWN.0 | Self::LOWER_LEFT.0 | Self::LEFT.0 | Self::UPPER_LEFT.0 | Self::UP.0);
    pub const LOWER_RIGHT_EDGE: Self = Self(Self::LEFT.0 | Self::UPPER_LEFT.0 | Self::UP.0);
    pub const DOWN_EDGE: Self =
        Self(Self::UP.0 | Self::UPPER_RIGHT.0 | Self::RIGHT.0 | Self::LEFT.0 | Self::UPPER_LEFT.0);
    pub const LOWER_LEFT_EDGE: Self = Self(Self::UP.0 | Self::UPPER_RIGHT.0 | Self::RIGHT.0);
    pub const LEFT_EDGE: Self =
        Self(Self::UP.0 | Self::UPPER_RIGHT.0 | Self::RIGHT.0 | Self::LOWER_RIGHT.0 | Self::DOWN.0);
    pub const UPPER_LEFT_EDGE: Self = Self(Self::RIGHT.0 | Self::LOWER_RIGHT.0 | Self::DOWN.0);
    pub const UPPER_RIGHT_CORNER: Self = Self(
        Self::UP.0
            | Self::UPPER_RIGHT.0
            | Self::RIGHT.0
            | Self::LOWER_RIGHT.0
            | Self::DOWN.0
            | Self::LEFT.0
            | Self::UPPER_LEFT.0,
    );
    pub const LOWER_RIGHT_CORNER: Self = Self(
        Self::UP.0
            | Self::UPPER_RIGHT.0
            | Self::RIGHT.0
            | Self::LOWER_RIGHT.0
            | Self::DOWN.0
            | Self::LOWER_LEFT.0
            | Self::LEFT.0,
    );
    pub const LOWER_LEFT_CORNER: Self = Self(
        Self::UP.0
            | Self::RIGHT.0
            | Self::LOWER_RIGHT.0
            | Self::DOWN.0
            | Self::LOWER_LEFT.0
            | Self::LEFT.0
            | Self::UPPER_LEFT.0,
    );
    pub const UPPER_LEFT_CORNER: Self = Self(
        Self::UP.0
            | Self::UPPER_RIGHT.0
            | Self::RIGHT.0
            | Self::DOWN.0
            | Self::LOWER_LEFT.0
            | Self::LEFT.0
            | Self::UPPER_LEFT.0,
    );

    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }
}

impl BitOr for TileAdjacencyMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for TileAdjacencyMask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for TileAdjacencyMask {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for TileAdjacencyMask {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl AsNode for TileMapNode {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl TileDefinition {
    #[must_use]
    pub fn with_texture(texture: &Texture) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_tile_definition_new_with_texture(texture.as_ptr())) }
    }

    #[must_use]
    pub fn with_texture_size(texture: &Texture, size: CGSize) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_tile_definition_new_with_texture_size(
                texture.as_ptr(),
                size.width,
                size.height,
            ))
        }
    }

    #[must_use]
    pub fn name(&self) -> Option<String> {
        unsafe { crate::error::take_string(ffi::sk_tile_definition_copy_name(self.ptr)) }
    }

    pub fn set_name(&self, name: &str) {
        if let Some(name) = cstring_from_str(name) {
            unsafe { ffi::sk_tile_definition_set_name(self.ptr, name.as_ptr()) };
        }
    }

    #[must_use]
    pub fn size(&self) -> CGSize {
        CGSize::new(
            unsafe { ffi::sk_tile_definition_get_size_w(self.ptr) },
            unsafe { ffi::sk_tile_definition_get_size_h(self.ptr) },
        )
    }

    pub fn set_size(&self, size: CGSize) {
        unsafe { ffi::sk_tile_definition_set_size(self.ptr, size.width, size.height) };
    }

    #[must_use]
    pub fn placement_weight(&self) -> u64 {
        unsafe { ffi::sk_tile_definition_get_placement_weight(self.ptr) }
    }

    pub fn set_placement_weight(&self, weight: u64) {
        unsafe { ffi::sk_tile_definition_set_placement_weight(self.ptr, weight) };
    }

    #[must_use]
    pub fn rotation(&self) -> TileDefinitionRotation {
        TileDefinitionRotation::from_raw(unsafe { ffi::sk_tile_definition_get_rotation(self.ptr) })
    }

    pub fn set_rotation(&self, rotation: TileDefinitionRotation) {
        unsafe { ffi::sk_tile_definition_set_rotation(self.ptr, rotation as u64) };
    }

    #[must_use]
    pub fn flip_vertically(&self) -> bool {
        unsafe { ffi::sk_tile_definition_get_flip_vertically(self.ptr) }
    }

    pub fn set_flip_vertically(&self, flip: bool) {
        unsafe { ffi::sk_tile_definition_set_flip_vertically(self.ptr, flip) };
    }

    #[must_use]
    pub fn flip_horizontally(&self) -> bool {
        unsafe { ffi::sk_tile_definition_get_flip_horizontally(self.ptr) }
    }

    pub fn set_flip_horizontally(&self, flip: bool) {
        unsafe { ffi::sk_tile_definition_set_flip_horizontally(self.ptr, flip) };
    }
}

impl TileGroupRule {
    #[must_use]
    pub fn new(adjacency: TileAdjacencyMask, tile_definitions: &[&TileDefinition]) -> Option<Self> {
        let mut raw: Vec<*mut c_void> = tile_definitions
            .iter()
            .map(|definition| definition.as_ptr())
            .collect();
        let raw_ptr = if raw.is_empty() {
            core::ptr::null_mut()
        } else {
            raw.as_mut_ptr().cast()
        };
        unsafe {
            Self::from_raw(ffi::sk_tile_group_rule_new(
                adjacency.bits(),
                raw_ptr,
                raw.len(),
            ))
        }
    }

    #[must_use]
    pub fn adjacency(&self) -> TileAdjacencyMask {
        TileAdjacencyMask(unsafe { ffi::sk_tile_group_rule_get_adjacency(self.ptr) })
    }

    pub fn set_adjacency(&self, adjacency: TileAdjacencyMask) {
        unsafe { ffi::sk_tile_group_rule_set_adjacency(self.ptr, adjacency.bits()) };
    }

    #[must_use]
    pub fn name(&self) -> Option<String> {
        unsafe { crate::error::take_string(ffi::sk_tile_group_rule_copy_name(self.ptr)) }
    }

    pub fn set_name(&self, name: &str) {
        if let Some(name) = cstring_from_str(name) {
            unsafe { ffi::sk_tile_group_rule_set_name(self.ptr, name.as_ptr()) };
        }
    }
}

impl TileGroup {
    #[must_use]
    pub fn with_tile_definition(tile_definition: &TileDefinition) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_tile_group_new_with_tile_definition(
                tile_definition.as_ptr(),
            ))
        }
    }

    #[must_use]
    pub fn with_rules(rules: &[&TileGroupRule]) -> Option<Self> {
        let mut raw: Vec<*mut c_void> = rules.iter().map(|rule| rule.as_ptr()).collect();
        let raw_ptr = if raw.is_empty() {
            core::ptr::null_mut()
        } else {
            raw.as_mut_ptr().cast()
        };
        unsafe { Self::from_raw(ffi::sk_tile_group_new_with_rules(raw_ptr, raw.len())) }
    }

    #[must_use]
    pub fn empty() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_tile_group_empty()) }
    }

    #[must_use]
    pub fn name(&self) -> Option<String> {
        unsafe { crate::error::take_string(ffi::sk_tile_group_copy_name(self.ptr)) }
    }

    pub fn set_name(&self, name: &str) {
        if let Some(name) = cstring_from_str(name) {
            unsafe { ffi::sk_tile_group_set_name(self.ptr, name.as_ptr()) };
        }
    }
}

impl TileSet {
    #[must_use]
    pub fn with_tile_groups(tile_groups: &[&TileGroup]) -> Option<Self> {
        let mut raw: Vec<*mut c_void> = tile_groups.iter().map(|group| group.as_ptr()).collect();
        let raw_ptr = if raw.is_empty() {
            core::ptr::null_mut()
        } else {
            raw.as_mut_ptr().cast()
        };
        unsafe { Self::from_raw(ffi::sk_tile_set_new(raw_ptr, raw.len())) }
    }

    #[must_use]
    pub fn with_tile_groups_type(
        tile_groups: &[&TileGroup],
        tile_set_type: TileSetType,
    ) -> Option<Self> {
        let mut raw: Vec<*mut c_void> = tile_groups.iter().map(|group| group.as_ptr()).collect();
        let raw_ptr = if raw.is_empty() {
            core::ptr::null_mut()
        } else {
            raw.as_mut_ptr().cast()
        };
        unsafe {
            Self::from_raw(ffi::sk_tile_set_new_with_type(
                raw_ptr,
                raw.len(),
                tile_set_type as u64,
            ))
        }
    }

    #[must_use]
    pub fn name(&self) -> Option<String> {
        unsafe { crate::error::take_string(ffi::sk_tile_set_copy_name(self.ptr)) }
    }

    pub fn set_name(&self, name: &str) {
        if let Some(name) = cstring_from_str(name) {
            unsafe { ffi::sk_tile_set_set_name(self.ptr, name.as_ptr()) };
        }
    }

    #[must_use]
    pub fn tile_set_type(&self) -> TileSetType {
        TileSetType::from_raw(unsafe { ffi::sk_tile_set_get_type(self.ptr) })
    }

    pub fn set_tile_set_type(&self, tile_set_type: TileSetType) {
        unsafe { ffi::sk_tile_set_set_type(self.ptr, tile_set_type as u64) };
    }

    #[must_use]
    pub fn default_tile_group(&self) -> Option<TileGroup> {
        unsafe { TileGroup::from_raw(ffi::sk_tile_set_get_default_tile_group(self.ptr)) }
    }

    pub fn set_default_tile_group(&self, tile_group: Option<&TileGroup>) {
        unsafe {
            ffi::sk_tile_set_set_default_tile_group(
                self.ptr,
                tile_group.map_or(core::ptr::null_mut(), TileGroup::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn default_tile_size(&self) -> CGSize {
        CGSize::new(
            unsafe { ffi::sk_tile_set_get_default_tile_size_w(self.ptr) },
            unsafe { ffi::sk_tile_set_get_default_tile_size_h(self.ptr) },
        )
    }

    pub fn set_default_tile_size(&self, size: CGSize) {
        unsafe { ffi::sk_tile_set_set_default_tile_size(self.ptr, size.width, size.height) };
    }
}

impl TileMapNode {
    #[must_use]
    pub fn new(tile_set: &TileSet, columns: usize, rows: usize, tile_size: CGSize) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_tile_map_node_new(
                tile_set.as_ptr(),
                columns,
                rows,
                tile_size.width,
                tile_size.height,
            ))
        }
    }

    #[must_use]
    pub fn with_fill(
        tile_set: &TileSet,
        columns: usize,
        rows: usize,
        tile_size: CGSize,
        tile_group: &TileGroup,
    ) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_tile_map_node_new_with_fill(
                tile_set.as_ptr(),
                columns,
                rows,
                tile_size.width,
                tile_size.height,
                tile_group.as_ptr(),
            ))
        }
    }

    #[must_use]
    pub fn number_of_columns(&self) -> usize {
        unsafe { ffi::sk_tile_map_node_get_number_of_columns(self.ptr) }
    }

    #[must_use]
    pub fn number_of_rows(&self) -> usize {
        unsafe { ffi::sk_tile_map_node_get_number_of_rows(self.ptr) }
    }

    #[must_use]
    pub fn tile_size(&self) -> CGSize {
        CGSize::new(
            unsafe { ffi::sk_tile_map_node_get_tile_size_w(self.ptr) },
            unsafe { ffi::sk_tile_map_node_get_tile_size_h(self.ptr) },
        )
    }

    pub fn set_tile_size(&self, size: CGSize) {
        unsafe { ffi::sk_tile_map_node_set_tile_size(self.ptr, size.width, size.height) };
    }

    #[must_use]
    pub fn map_size(&self) -> CGSize {
        CGSize::new(
            unsafe { ffi::sk_tile_map_node_get_map_size_w(self.ptr) },
            unsafe { ffi::sk_tile_map_node_get_map_size_h(self.ptr) },
        )
    }

    #[must_use]
    pub fn tile_set(&self) -> Option<TileSet> {
        unsafe { TileSet::from_raw(ffi::sk_tile_map_node_get_tile_set(self.ptr)) }
    }

    pub fn set_tile_set(&self, tile_set: &TileSet) {
        unsafe { ffi::sk_tile_map_node_set_tile_set(self.ptr, tile_set.as_ptr()) };
    }

    pub fn fill_with_tile_group(&self, tile_group: Option<&TileGroup>) {
        unsafe {
            ffi::sk_tile_map_node_fill_with_tile_group(
                self.ptr,
                tile_group.map_or(core::ptr::null_mut(), TileGroup::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn tile_group_at(&self, column: usize, row: usize) -> Option<TileGroup> {
        unsafe { TileGroup::from_raw(ffi::sk_tile_map_node_get_tile_group(self.ptr, column, row)) }
    }

    pub fn set_tile_group(&self, tile_group: Option<&TileGroup>, column: usize, row: usize) {
        unsafe {
            ffi::sk_tile_map_node_set_tile_group(
                self.ptr,
                tile_group.map_or(core::ptr::null_mut(), TileGroup::as_ptr),
                column,
                row,
            );
        };
    }

    #[must_use]
    pub fn tile_definition_at(&self, column: usize, row: usize) -> Option<TileDefinition> {
        unsafe {
            TileDefinition::from_raw(ffi::sk_tile_map_node_get_tile_definition(
                self.ptr, column, row,
            ))
        }
    }

    #[must_use]
    pub fn center_of_tile_at(&self, column: usize, row: usize) -> CGPoint {
        CGPoint::new(
            unsafe { ffi::sk_tile_map_node_center_of_tile_x(self.ptr, column, row) },
            unsafe { ffi::sk_tile_map_node_center_of_tile_y(self.ptr, column, row) },
        )
    }

    #[must_use]
    pub fn tile_column_index_from_position(&self, position: CGPoint) -> usize {
        unsafe {
            ffi::sk_tile_map_node_tile_column_index_from_position(self.ptr, position.x, position.y)
        }
    }

    #[must_use]
    pub fn tile_row_index_from_position(&self, position: CGPoint) -> usize {
        unsafe {
            ffi::sk_tile_map_node_tile_row_index_from_position(self.ptr, position.x, position.y)
        }
    }
}
