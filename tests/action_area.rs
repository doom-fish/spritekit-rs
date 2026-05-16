use spritekit::{Action, ActionTimingMode};

#[test]
fn action_exposes_timing_properties() {
    let action = Action::move_by(1.0, 2.0, 0.25).expect("action");
    action.set_duration(0.75);
    action.set_speed(1.5);
    action.set_timing_mode(ActionTimingMode::EaseOut);

    assert!((action.duration() - 0.75).abs() < f64::EPSILON);
    assert!((action.speed() - 1.5).abs() < f64::EPSILON);
    assert_eq!(action.timing_mode(), ActionTimingMode::EaseOut);
    assert!(action.reversed().is_some());
}
