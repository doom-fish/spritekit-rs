use crate::color::Color;
use crate::ffi;
use crate::private::handle_type;

handle_type!(Transition);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum TransitionDirection {
    #[default]
    Up = 0,
    Down = 1,
    Right = 2,
    Left = 3,
}

impl TransitionDirection {
    #[must_use]
    pub const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Down,
            2 => Self::Right,
            3 => Self::Left,
            _ => Self::Up,
        }
    }
}

impl Transition {
    #[must_use]
    pub fn cross_fade(duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_transition_cross_fade(duration)) }
    }

    #[must_use]
    pub fn fade(duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_transition_fade(duration)) }
    }

    #[must_use]
    pub fn fade_with_color(color: Color, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_transition_fade_with_color(color.r, color.g, color.b, color.a, duration)) }
    }

    #[must_use]
    pub fn flip_horizontal(duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_transition_flip_horizontal(duration)) }
    }

    #[must_use]
    pub fn flip_vertical(duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_transition_flip_vertical(duration)) }
    }

    #[must_use]
    pub fn reveal(direction: TransitionDirection, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_transition_reveal(direction as i32, duration)) }
    }

    #[must_use]
    pub fn move_in(direction: TransitionDirection, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_transition_move_in(direction as i32, duration)) }
    }

    #[must_use]
    pub fn push(direction: TransitionDirection, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_transition_push(direction as i32, duration)) }
    }

    #[must_use]
    pub fn doors_open_horizontal(duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_transition_doors_open_horizontal(duration)) }
    }

    #[must_use]
    pub fn doors_open_vertical(duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_transition_doors_open_vertical(duration)) }
    }

    #[must_use]
    pub fn doors_close_horizontal(duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_transition_doors_close_horizontal(duration)) }
    }

    #[must_use]
    pub fn doors_close_vertical(duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_transition_doors_close_vertical(duration)) }
    }

    #[must_use]
    pub fn doorway(duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_transition_doorway(duration)) }
    }

    #[must_use]
    pub fn pauses_incoming_scene(&self) -> bool {
        unsafe { ffi::sk_transition_get_pauses_incoming_scene(self.ptr) }
    }

    pub fn set_pauses_incoming_scene(&self, pauses: bool) {
        unsafe { ffi::sk_transition_set_pauses_incoming_scene(self.ptr, pauses) };
    }

    #[must_use]
    pub fn pauses_outgoing_scene(&self) -> bool {
        unsafe { ffi::sk_transition_get_pauses_outgoing_scene(self.ptr) }
    }

    pub fn set_pauses_outgoing_scene(&self, pauses: bool) {
        unsafe { ffi::sk_transition_set_pauses_outgoing_scene(self.ptr, pauses) };
    }
}
