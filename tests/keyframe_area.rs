use spritekit::{InterpolationMode, KeyframeSequence, RepeatMode};

#[test]
fn keyframe_sequence_samples_values() {
    let sequence = KeyframeSequence::from_scalars(&[(0.0, 0.0), (1.0, 10.0)]).expect("sequence");
    sequence.set_interpolation_mode(InterpolationMode::Linear);
    sequence.set_repeat_mode(RepeatMode::Clamp);
    sequence.set_scalar_keyframe_value(1, 12.0);

    let sample = sequence.sample_scalar(1.0).expect("sample");
    assert_eq!(sequence.count(), 2);
    assert_eq!(sequence.interpolation_mode(), InterpolationMode::Linear);
    assert_eq!(sequence.repeat_mode(), RepeatMode::Clamp);
    assert!((sample - 12.0).abs() < 1e-6);
}
