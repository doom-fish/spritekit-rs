use spritekit::{Action, ActionTimingMode};

fn main() {
    let move_action = Action::move_by(10.0, 5.0, 0.25).expect("move action");
    let wait_action = Action::wait(0.1).expect("wait action");
    let sequence = Action::sequence(&[&move_action, &wait_action]).expect("sequence");

    sequence.set_duration(0.5);
    sequence.set_speed(2.0);
    sequence.set_timing_mode(ActionTimingMode::EaseInEaseOut);
    let reversed = sequence.reversed().expect("reversed action");

    assert!((sequence.duration() - 0.5).abs() < f64::EPSILON);
    assert!((sequence.speed() - 2.0).abs() < f64::EPSILON);
    assert_eq!(sequence.timing_mode(), ActionTimingMode::EaseInEaseOut);
    assert!(reversed.duration() >= 0.0);

    println!("action basics ok");
}
