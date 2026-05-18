use core::ffi::{c_char, c_void};

extern "C" {
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_new_with_text(text: *const c_char) -> *mut c_void;
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_new_with_font_named(font_name: *const c_char) -> *mut c_void;
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_copy_font_name(node: *mut c_void) -> *mut c_char;
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_set_font_name(node: *mut c_void, font_name: *const c_char);
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_copy_text(node: *mut c_void) -> *mut c_char;
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_set_text(node: *mut c_void, text: *const c_char);
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_get_font_size(node: *mut c_void) -> f64;
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_set_font_size(node: *mut c_void, font_size: f64);
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_get_vertical_alignment_mode(node: *mut c_void) -> i32;
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_set_vertical_alignment_mode(node: *mut c_void, mode: i32);
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_get_horizontal_alignment_mode(node: *mut c_void) -> i32;
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_set_horizontal_alignment_mode(node: *mut c_void, mode: i32);
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_get_number_of_lines(node: *mut c_void) -> isize;
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_set_number_of_lines(node: *mut c_void, lines: isize);
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_get_preferred_max_layout_width(node: *mut c_void) -> f64;
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_set_preferred_max_layout_width(node: *mut c_void, width: f64);
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_set_font_color(node: *mut c_void, r: f32, g: f32, b: f32, a: f32);
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_get_color_blend_factor(node: *mut c_void) -> f64;
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_set_color_blend_factor(node: *mut c_void, factor: f64);
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_set_color(node: *mut c_void, r: f32, g: f32, b: f32, a: f32);
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_get_blend_mode(node: *mut c_void) -> i32;
    /// Wraps `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
    pub fn sk_label_node_set_blend_mode(node: *mut c_void, mode: i32);
}
