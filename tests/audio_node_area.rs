use spritekit::AudioNode;

#[test]
fn audio_node_exposes_playback_flags() {
    let audio = AudioNode::new().expect("audio node");
    audio.set_autoplay_looped(false);
    audio.set_positional(true);

    assert!(!audio.autoplay_looped());
    assert!(audio.is_positional());
    assert!(audio.has_audio_node());
}
