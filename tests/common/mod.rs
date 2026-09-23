#![allow(dead_code)]

use core::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use apple_metal::MetalDevice;
use spritekit::{
    CGPoint, CGSize, CGVector, Node, NodeExt, PhysicsBody, PhysicsContact, PhysicsContactDelegate,
    Renderer, Scene,
};

extern "C" {
    fn objc_autoreleasePoolPush() -> *mut c_void;
    fn objc_autoreleasePoolPop(pool: *mut c_void);
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFRunLoopDefaultMode: *const c_void;
    fn CFRunLoopRunInMode(mode: *const c_void, seconds: f64, return_after_source: u8) -> i32;
}

pub fn drain_main_queue() {
    unsafe { CFRunLoopRunInMode(kCFRunLoopDefaultMode, 0.2, 0) };
}

pub fn autoreleasepool<R>(body: impl FnOnce() -> R) -> R {
    let pool = unsafe { objc_autoreleasePoolPush() };
    let result = body();
    unsafe { objc_autoreleasePoolPop(pool) };
    result
}

pub const FRAME: f64 = 1.0 / 60.0;

pub struct Stage {
    pub scene: Scene,
    renderer: Renderer,
    _device: MetalDevice,
    time: f64,
}

impl Stage {
    pub fn new(width: f64, height: f64) -> Self {
        let device = MetalDevice::system_default().expect("Metal device");
        let renderer = Renderer::new(&device).expect("SKRenderer");
        let scene = Scene::with_size(CGSize::new(width, height)).expect("scene");
        renderer.set_scene(Some(&scene));
        Self {
            scene,
            renderer,
            _device: device,
            time: 1.0,
        }
    }

    pub fn step(&mut self, frames: usize) {
        for _ in 0..frames {
            autoreleasepool(|| self.renderer.update_at_time(self.time));
            self.time += FRAME;
        }
    }
}

pub fn add_overlapping_bodies(stage: &Stage) {
    stage
        .scene
        .physics_world()
        .set_gravity(CGVector::new(0.0, 0.0));
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
    }
}

pub fn counting_delegate(begins: &Arc<AtomicUsize>) -> PhysicsContactDelegate {
    let begins = Arc::clone(begins);
    PhysicsContactDelegate::from_callbacks(
        Some(move |contact: PhysicsContact| {
            assert!(contact.body_a().is_some());
            assert!(contact.body_b().is_some());
            assert!(contact.contact_point().x.is_finite());
            begins.fetch_add(1, Ordering::SeqCst);
        }),
        None::<fn(PhysicsContact)>,
    )
    .expect("contact delegate")
}
