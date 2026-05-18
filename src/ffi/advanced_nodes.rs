use core::ffi::{c_char, c_void};

extern "C" {
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_camera_node_new() -> *mut c_void;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_camera_node_contains_node(camera: *mut c_void, node: *mut c_void) -> bool;
    /// Returns a property exposed by `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_camera_node_get_contained_node_count(camera: *mut c_void) -> usize;

    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_crop_node_new() -> *mut c_void;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_crop_node_get_mask_node(node: *mut c_void) -> *mut c_void;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_crop_node_set_mask_node(node: *mut c_void, mask_node: *mut c_void);

    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_field_node_drag() -> *mut c_void;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_field_node_vortex() -> *mut c_void;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_field_node_linear_gravity(x: f32, y: f32, z: f32) -> *mut c_void;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_field_node_get_region(node: *mut c_void) -> *mut c_void;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_field_node_set_region(node: *mut c_void, region: *mut c_void);
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_field_node_get_strength(node: *mut c_void) -> f32;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_field_node_set_strength(node: *mut c_void, value: f32);
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_field_node_get_direction_x(node: *mut c_void) -> f32;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_field_node_get_direction_y(node: *mut c_void) -> f32;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_field_node_get_direction_z(node: *mut c_void) -> f32;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_field_node_set_direction(node: *mut c_void, x: f32, y: f32, z: f32);
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_field_node_get_enabled(node: *mut c_void) -> bool;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_field_node_set_enabled(node: *mut c_void, enabled: bool);

    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_reference_node_new_with_file_named(file_name: *const c_char) -> *mut c_void;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_reference_node_new_with_url_path(path: *const c_char) -> *mut c_void;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_reference_node_resolve(node: *mut c_void);

    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_shape_node_new_with_rect(x: f64, y: f64, width: f64, height: f64) -> *mut c_void;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_shape_node_new_with_rect_of_size(width: f64, height: f64) -> *mut c_void;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_shape_node_new_with_circle(radius: f64) -> *mut c_void;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_shape_node_get_line_width(node: *mut c_void) -> f64;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_shape_node_set_line_width(node: *mut c_void, value: f64);
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_shape_node_get_glow_width(node: *mut c_void) -> f64;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_shape_node_set_glow_width(node: *mut c_void, value: f64);
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_shape_node_get_antialiased(node: *mut c_void) -> bool;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_shape_node_set_antialiased(node: *mut c_void, antialiased: bool);
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_shape_node_get_line_length(node: *mut c_void) -> f64;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_shape_node_set_stroke_color(node: *mut c_void, r: f32, g: f32, b: f32, a: f32);
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_shape_node_set_fill_color(node: *mut c_void, r: f32, g: f32, b: f32, a: f32);

    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_transform_node_new() -> *mut c_void;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_transform_node_get_x_rotation(node: *mut c_void) -> f64;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_transform_node_set_x_rotation(node: *mut c_void, value: f64);
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_transform_node_get_y_rotation(node: *mut c_void) -> f64;
    /// Wraps `SKCameraNode`, `SKCropNode`, `SKFieldNode`, `SKReferenceNode`, `SKShapeNode`, and `SKTransformNode`.
    pub fn sk_transform_node_set_y_rotation(node: *mut c_void, value: f64);
}
