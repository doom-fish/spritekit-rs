use crate::ffi;
use crate::node::AsNode;
use crate::private::handle_type;

handle_type!(AudioNode);

impl AsNode for AudioNode {
    fn as_node_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl AudioNode {
    /// Wraps `SKAudioNode`.
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_audio_node_new()) }
    }

    /// Returns a property exposed by `SKAudioNode`.
    #[must_use]
    pub fn autoplay_looped(&self) -> bool {
        unsafe { ffi::sk_audio_node_get_autoplay_looped(self.ptr) }
    }

    /// Sets a property exposed by `SKAudioNode`.
    pub fn set_autoplay_looped(&self, autoplay: bool) {
        unsafe { ffi::sk_audio_node_set_autoplay_looped(self.ptr, autoplay) };
    }

    /// Returns a property exposed by `SKAudioNode`.
    #[must_use]
    pub fn is_positional(&self) -> bool {
        unsafe { ffi::sk_audio_node_get_positional(self.ptr) }
    }

    /// Sets a property exposed by `SKAudioNode`.
    pub fn set_positional(&self, positional: bool) {
        unsafe { ffi::sk_audio_node_set_positional(self.ptr, positional) };
    }

    /// Returns a property exposed by `SKAudioNode`.
    #[must_use]
    pub fn has_audio_node(&self) -> bool {
        unsafe { ffi::sk_audio_node_has_av_audio_node(self.ptr) }
    }
}

impl Default for AudioNode {
    fn default() -> Self {
        Self::new().expect("SKAudioNode creation failed")
    }
}
