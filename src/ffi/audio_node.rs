use core::ffi::c_void;

extern "C" {
    /// Wraps `SKAudioNode`.
    pub fn sk_audio_node_new() -> *mut c_void;
    /// Wraps `SKAudioNode`.
    pub fn sk_audio_node_get_autoplay_looped(node: *mut c_void) -> bool;
    /// Wraps `SKAudioNode`.
    pub fn sk_audio_node_set_autoplay_looped(node: *mut c_void, autoplay: bool);
    /// Wraps `SKAudioNode`.
    pub fn sk_audio_node_get_positional(node: *mut c_void) -> bool;
    /// Wraps `SKAudioNode`.
    pub fn sk_audio_node_set_positional(node: *mut c_void, positional: bool);
    /// Wraps `SKAudioNode`.
    pub fn sk_audio_node_has_av_audio_node(node: *mut c_void) -> bool;
}
