use core::ffi::c_void;

extern "C" {
    pub fn sk_transition_cross_fade(duration: f64) -> *mut c_void;
    pub fn sk_transition_fade(duration: f64) -> *mut c_void;
    pub fn sk_transition_fade_with_color(
        r: f32,
        g: f32,
        b: f32,
        a: f32,
        duration: f64,
    ) -> *mut c_void;
    pub fn sk_transition_flip_horizontal(duration: f64) -> *mut c_void;
    pub fn sk_transition_flip_vertical(duration: f64) -> *mut c_void;
    pub fn sk_transition_reveal(direction: i32, duration: f64) -> *mut c_void;
    pub fn sk_transition_move_in(direction: i32, duration: f64) -> *mut c_void;
    pub fn sk_transition_push(direction: i32, duration: f64) -> *mut c_void;
    pub fn sk_transition_doors_open_horizontal(duration: f64) -> *mut c_void;
    pub fn sk_transition_doors_open_vertical(duration: f64) -> *mut c_void;
    pub fn sk_transition_doors_close_horizontal(duration: f64) -> *mut c_void;
    pub fn sk_transition_doors_close_vertical(duration: f64) -> *mut c_void;
    pub fn sk_transition_doorway(duration: f64) -> *mut c_void;
    pub fn sk_transition_get_pauses_incoming_scene(transition: *mut c_void) -> bool;
    pub fn sk_transition_set_pauses_incoming_scene(transition: *mut c_void, pauses: bool);
    pub fn sk_transition_get_pauses_outgoing_scene(transition: *mut c_void) -> bool;
    pub fn sk_transition_set_pauses_outgoing_scene(transition: *mut c_void, pauses: bool);
}
