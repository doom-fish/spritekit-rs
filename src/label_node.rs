use crate::color::Color;
use crate::ffi;
use crate::node::AsNode;
use crate::physics::BlendMode;
use crate::private::{cstring_from_str, handle_type};

handle_type!(LabelNode);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum VerticalAlignmentMode {
    #[default]
    Baseline = 0,
    Center = 1,
    Top = 2,
    Bottom = 3,
}

impl VerticalAlignmentMode {
    #[must_use]
    pub const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Center,
            2 => Self::Top,
            3 => Self::Bottom,
            _ => Self::Baseline,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum HorizontalAlignmentMode {
    #[default]
    Center = 0,
    Left = 1,
    Right = 2,
}

impl HorizontalAlignmentMode {
    #[must_use]
    pub const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Left,
            2 => Self::Right,
            _ => Self::Center,
        }
    }
}

impl AsNode for LabelNode {
    fn as_node_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl LabelNode {
    #[must_use]
    pub fn new() -> Option<Self> {
        Self::with_text("")
    }

    #[must_use]
    pub fn with_text(text: &str) -> Option<Self> {
        let text = cstring_from_str(text)?;
        unsafe { Self::from_raw(ffi::sk_label_node_new_with_text(text.as_ptr())) }
    }

    #[must_use]
    pub fn with_font_named(font_name: &str) -> Option<Self> {
        let font_name = cstring_from_str(font_name)?;
        unsafe { Self::from_raw(ffi::sk_label_node_new_with_font_named(font_name.as_ptr())) }
    }

    #[must_use]
    pub fn font_name(&self) -> Option<String> {
        unsafe { crate::error::take_string(ffi::sk_label_node_copy_font_name(self.ptr)) }
    }

    pub fn set_font_name(&self, font_name: &str) {
        if let Some(font_name) = cstring_from_str(font_name) {
            unsafe { ffi::sk_label_node_set_font_name(self.ptr, font_name.as_ptr()) };
        }
    }

    #[must_use]
    pub fn text(&self) -> Option<String> {
        unsafe { crate::error::take_string(ffi::sk_label_node_copy_text(self.ptr)) }
    }

    pub fn set_text(&self, text: &str) {
        if let Some(text) = cstring_from_str(text) {
            unsafe { ffi::sk_label_node_set_text(self.ptr, text.as_ptr()) };
        }
    }

    #[must_use]
    pub fn font_size(&self) -> f64 {
        unsafe { ffi::sk_label_node_get_font_size(self.ptr) }
    }

    pub fn set_font_size(&self, font_size: f64) {
        unsafe { ffi::sk_label_node_set_font_size(self.ptr, font_size) };
    }

    #[must_use]
    pub fn vertical_alignment_mode(&self) -> VerticalAlignmentMode {
        VerticalAlignmentMode::from_raw(unsafe {
            ffi::sk_label_node_get_vertical_alignment_mode(self.ptr)
        })
    }

    pub fn set_vertical_alignment_mode(&self, mode: VerticalAlignmentMode) {
        unsafe { ffi::sk_label_node_set_vertical_alignment_mode(self.ptr, mode as i32) };
    }

    #[must_use]
    pub fn horizontal_alignment_mode(&self) -> HorizontalAlignmentMode {
        HorizontalAlignmentMode::from_raw(unsafe {
            ffi::sk_label_node_get_horizontal_alignment_mode(self.ptr)
        })
    }

    pub fn set_horizontal_alignment_mode(&self, mode: HorizontalAlignmentMode) {
        unsafe { ffi::sk_label_node_set_horizontal_alignment_mode(self.ptr, mode as i32) };
    }

    #[must_use]
    pub fn number_of_lines(&self) -> usize {
        usize::try_from(unsafe { ffi::sk_label_node_get_number_of_lines(self.ptr) })
            .unwrap_or_default()
    }

    pub fn set_number_of_lines(&self, lines: usize) {
        let lines = isize::try_from(lines).unwrap_or(isize::MAX);
        unsafe { ffi::sk_label_node_set_number_of_lines(self.ptr, lines) };
    }

    #[must_use]
    pub fn preferred_max_layout_width(&self) -> f64 {
        unsafe { ffi::sk_label_node_get_preferred_max_layout_width(self.ptr) }
    }

    pub fn set_preferred_max_layout_width(&self, width: f64) {
        unsafe { ffi::sk_label_node_set_preferred_max_layout_width(self.ptr, width) };
    }

    pub fn set_font_color(&self, color: Color) {
        unsafe { ffi::sk_label_node_set_font_color(self.ptr, color.r, color.g, color.b, color.a) };
    }

    #[must_use]
    pub fn color_blend_factor(&self) -> f64 {
        unsafe { ffi::sk_label_node_get_color_blend_factor(self.ptr) }
    }

    pub fn set_color_blend_factor(&self, factor: f64) {
        unsafe { ffi::sk_label_node_set_color_blend_factor(self.ptr, factor) };
    }

    pub fn set_color(&self, color: Color) {
        unsafe { ffi::sk_label_node_set_color(self.ptr, color.r, color.g, color.b, color.a) };
    }

    #[must_use]
    pub fn blend_mode(&self) -> BlendMode {
        BlendMode::from_raw(unsafe { ffi::sk_label_node_get_blend_mode(self.ptr) })
    }

    pub fn set_blend_mode(&self, mode: BlendMode) {
        unsafe { ffi::sk_label_node_set_blend_mode(self.ptr, mode as i32) };
    }
}

impl Default for LabelNode {
    fn default() -> Self {
        Self::new().expect("SKLabelNode creation failed")
    }
}
