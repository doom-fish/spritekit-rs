use core::ffi::{c_char, c_void};

extern "C" {
    pub fn sk_tile_definition_new_with_texture(texture: *mut c_void) -> *mut c_void;
    pub fn sk_tile_definition_new_with_texture_size(texture: *mut c_void, width: f64, height: f64) -> *mut c_void;
    pub fn sk_tile_definition_copy_name(definition: *mut c_void) -> *mut c_char;
    pub fn sk_tile_definition_set_name(definition: *mut c_void, name: *const c_char);
    pub fn sk_tile_definition_get_size_w(definition: *mut c_void) -> f64;
    pub fn sk_tile_definition_get_size_h(definition: *mut c_void) -> f64;
    pub fn sk_tile_definition_set_size(definition: *mut c_void, width: f64, height: f64);
    pub fn sk_tile_definition_get_placement_weight(definition: *mut c_void) -> u64;
    pub fn sk_tile_definition_set_placement_weight(definition: *mut c_void, weight: u64);
    pub fn sk_tile_definition_get_rotation(definition: *mut c_void) -> u64;
    pub fn sk_tile_definition_set_rotation(definition: *mut c_void, rotation: u64);
    pub fn sk_tile_definition_get_flip_vertically(definition: *mut c_void) -> bool;
    pub fn sk_tile_definition_set_flip_vertically(definition: *mut c_void, flip: bool);
    pub fn sk_tile_definition_get_flip_horizontally(definition: *mut c_void) -> bool;
    pub fn sk_tile_definition_set_flip_horizontally(definition: *mut c_void, flip: bool);

    pub fn sk_tile_group_rule_new(adjacency: u64, tile_definitions: *mut c_void, count: usize) -> *mut c_void;
    pub fn sk_tile_group_rule_get_adjacency(rule: *mut c_void) -> u64;
    pub fn sk_tile_group_rule_set_adjacency(rule: *mut c_void, adjacency: u64);
    pub fn sk_tile_group_rule_copy_name(rule: *mut c_void) -> *mut c_char;
    pub fn sk_tile_group_rule_set_name(rule: *mut c_void, name: *const c_char);

    pub fn sk_tile_group_new_with_tile_definition(tile_definition: *mut c_void) -> *mut c_void;
    pub fn sk_tile_group_new_with_rules(rules: *mut c_void, count: usize) -> *mut c_void;
    pub fn sk_tile_group_empty() -> *mut c_void;
    pub fn sk_tile_group_copy_name(group: *mut c_void) -> *mut c_char;
    pub fn sk_tile_group_set_name(group: *mut c_void, name: *const c_char);

    pub fn sk_tile_set_new(tile_groups: *mut c_void, count: usize) -> *mut c_void;
    pub fn sk_tile_set_new_with_type(tile_groups: *mut c_void, count: usize, tile_set_type: u64) -> *mut c_void;
    pub fn sk_tile_set_copy_name(tile_set: *mut c_void) -> *mut c_char;
    pub fn sk_tile_set_set_name(tile_set: *mut c_void, name: *const c_char);
    pub fn sk_tile_set_get_type(tile_set: *mut c_void) -> u64;
    pub fn sk_tile_set_set_type(tile_set: *mut c_void, tile_set_type: u64);
    pub fn sk_tile_set_get_default_tile_group(tile_set: *mut c_void) -> *mut c_void;
    pub fn sk_tile_set_set_default_tile_group(tile_set: *mut c_void, tile_group: *mut c_void);
    pub fn sk_tile_set_get_default_tile_size_w(tile_set: *mut c_void) -> f64;
    pub fn sk_tile_set_get_default_tile_size_h(tile_set: *mut c_void) -> f64;
    pub fn sk_tile_set_set_default_tile_size(tile_set: *mut c_void, width: f64, height: f64);

    pub fn sk_tile_map_node_new(tile_set: *mut c_void, columns: usize, rows: usize, tile_width: f64, tile_height: f64) -> *mut c_void;
    pub fn sk_tile_map_node_new_with_fill(tile_set: *mut c_void, columns: usize, rows: usize, tile_width: f64, tile_height: f64, tile_group: *mut c_void) -> *mut c_void;
    pub fn sk_tile_map_node_get_number_of_columns(node: *mut c_void) -> usize;
    pub fn sk_tile_map_node_get_number_of_rows(node: *mut c_void) -> usize;
    pub fn sk_tile_map_node_get_tile_size_w(node: *mut c_void) -> f64;
    pub fn sk_tile_map_node_get_tile_size_h(node: *mut c_void) -> f64;
    pub fn sk_tile_map_node_set_tile_size(node: *mut c_void, width: f64, height: f64);
    pub fn sk_tile_map_node_get_map_size_w(node: *mut c_void) -> f64;
    pub fn sk_tile_map_node_get_map_size_h(node: *mut c_void) -> f64;
    pub fn sk_tile_map_node_get_tile_set(node: *mut c_void) -> *mut c_void;
    pub fn sk_tile_map_node_set_tile_set(node: *mut c_void, tile_set: *mut c_void);
    pub fn sk_tile_map_node_fill_with_tile_group(node: *mut c_void, tile_group: *mut c_void);
    pub fn sk_tile_map_node_get_tile_group(node: *mut c_void, column: usize, row: usize) -> *mut c_void;
    pub fn sk_tile_map_node_set_tile_group(node: *mut c_void, tile_group: *mut c_void, column: usize, row: usize);
    pub fn sk_tile_map_node_get_tile_definition(node: *mut c_void, column: usize, row: usize) -> *mut c_void;
    pub fn sk_tile_map_node_center_of_tile_x(node: *mut c_void, column: usize, row: usize) -> f64;
    pub fn sk_tile_map_node_center_of_tile_y(node: *mut c_void, column: usize, row: usize) -> f64;
    pub fn sk_tile_map_node_tile_column_index_from_position(node: *mut c_void, x: f64, y: f64) -> usize;
    pub fn sk_tile_map_node_tile_row_index_from_position(node: *mut c_void, x: f64, y: f64) -> usize;
}
