use core::ffi::c_void;

extern "C" {
    pub fn sk_audio_node_new() -> *mut c_void;
    pub fn sk_audio_node_get_autoplay_looped(node: *mut c_void) -> bool;
    pub fn sk_audio_node_set_autoplay_looped(node: *mut c_void, autoplay: bool);
    pub fn sk_audio_node_get_positional(node: *mut c_void) -> bool;
    pub fn sk_audio_node_set_positional(node: *mut c_void, positional: bool);
    pub fn sk_audio_node_has_av_audio_node(node: *mut c_void) -> bool;
}
