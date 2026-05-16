use spritekit::{InterpolationMode, KeyframeSequence, RepeatMode};

fn main() {
    let sequence = KeyframeSequence::from_scalars(&[(0.0, 0.0), (0.5, 10.0), (1.0, 20.0)])
        .expect("keyframe sequence");
    sequence.set_interpolation_mode(InterpolationMode::Spline);
    sequence.set_repeat_mode(RepeatMode::Loop);
    sequence.set_scalar_keyframe_value(1, 12.0);

    let sample = sequence.sample_scalar(0.5).expect("sample");
    assert_eq!(sequence.count(), 3);
    assert_eq!(sequence.interpolation_mode(), InterpolationMode::Spline);
    assert_eq!(sequence.repeat_mode(), RepeatMode::Loop);
    assert!(sample >= 0.0);

    println!("keyframe sequence ok");
}
