use core::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use spritekit::{
    CGPoint, CGVector, Node, NodeExt, PhysicsBody, PhysicsContact, PhysicsContactDelegate,
};

mod common;

use common::{autoreleasepool, Stage};

const SWAPS: usize = 50_000;

extern "C" {
    fn sk_physics_world_set_contact_delegate(world: *mut c_void, delegate: *mut c_void);
}

fn counting(calls: &Arc<AtomicUsize>) -> PhysicsContactDelegate {
    let begins = Arc::clone(calls);
    let ends = Arc::clone(calls);
    PhysicsContactDelegate::from_callbacks(
        Some(move |_: PhysicsContact| {
            begins.fetch_add(1, Ordering::Relaxed);
        }),
        Some(move |_: PhysicsContact| {
            ends.fetch_add(1, Ordering::Relaxed);
        }),
    )
    .expect("contact delegate")
}

#[test]
fn clearing_a_contact_delegate_while_another_thread_simulates_is_safe() {
    let mut stage = Stage::new(256.0, 256.0);
    stage
        .scene
        .physics_world()
        .set_gravity(CGVector::new(0.0, 0.0));
    let mut movers = Vec::new();
    for (category, x) in [(1_u32, 60.0), (2_u32, 66.0)] {
        let node = Node::new().expect("node");
        node.set_position(CGPoint::new(x, 64.0));
        let body = PhysicsBody::circle(8.0).expect("body");
        body.set_affected_by_gravity(false);
        body.set_category_bitmask(category);
        body.set_contact_test_bitmask(0b11 ^ category);
        body.set_collision_bitmask(0);
        node.set_physics_body(Some(&body));
        stage.scene.add_child(&node).expect("add body node");
        movers.push(node);
    }

    let installed_calls = Arc::new(AtomicUsize::new(0));
    let installed = counting(&installed_calls);
    let world = stage.scene.physics_world();
    world.set_contact_delegate(Some(&installed));
    let mover = &movers[1];
    for frame in 0..4 {
        let x = if frame % 2 == 0 { 200.0 } else { 66.0 };
        mover.set_position(CGPoint::new(x, 64.0));
        stage.step(1);
    }
    assert!(
        installed_calls.load(Ordering::Relaxed) > 0,
        "moving the body in and out of contact reaches the delegate"
    );

    let world_address = world.as_ptr() as usize;
    let calls = Arc::new(AtomicUsize::new(0));
    let swaps = Arc::new(AtomicUsize::new(0));
    let swapper = thread::spawn({
        let calls = Arc::clone(&calls);
        let swaps = Arc::clone(&swaps);
        move || {
            let world = world_address as *mut c_void;
            while swaps.load(Ordering::Relaxed) < SWAPS {
                autoreleasepool(|| {
                    let delegate = counting(&calls);
                    unsafe { sk_physics_world_set_contact_delegate(world, delegate.as_ptr()) };
                    thread::yield_now();
                    unsafe { sk_physics_world_set_contact_delegate(world, core::ptr::null_mut()) };
                    drop(delegate);
                });
                swaps.fetch_add(1, Ordering::Relaxed);
            }
        }
    });

    let started = Instant::now();
    let mut frame = 0_usize;
    while !swapper.is_finished() && started.elapsed() < Duration::from_secs(30) {
        let x = if frame % 2 == 0 { 200.0 } else { 66.0 };
        mover.set_position(CGPoint::new(x, 64.0));
        stage.step(1);
        frame += 1;
    }
    swapper.join().expect("swapper thread");
    assert!(frame > 0);
    assert_eq!(swaps.load(Ordering::Relaxed), SWAPS);
    assert!(
        calls.load(Ordering::Relaxed) > 0,
        "the simulation never called a delegate that another thread was swapping"
    );
    world.set_contact_delegate(None);
    drop(installed);
}
