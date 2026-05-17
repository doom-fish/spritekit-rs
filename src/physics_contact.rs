use core::ffi::c_void;
use std::panic::{catch_unwind, AssertUnwindSafe};

use apple_cf::cg::{CGPoint, CGVector};

use crate::ffi;
use crate::physics_body::PhysicsBody;
use crate::private::handle_type;

handle_type!(PhysicsContact);
handle_type!(PhysicsContactDelegate);

type ContactCallback = Box<dyn Fn(PhysicsContact) + Send + Sync + 'static>;

struct PhysicsContactDelegateContext {
    did_begin: Option<ContactCallback>,
    did_end: Option<ContactCallback>,
}

extern "C" fn physics_contact_delegate_did_begin(
    context: *mut c_void,
    contact_handle: *mut c_void,
) {
    let context = unsafe { &*(context.cast::<PhysicsContactDelegateContext>()) };
    let Some(callback) = &context.did_begin else {
        if !contact_handle.is_null() {
            unsafe { ffi::sk_release(contact_handle) };
        }
        return;
    };
    if let Some(contact) = unsafe { PhysicsContact::from_raw(contact_handle) } {
        let _ = catch_unwind(AssertUnwindSafe(|| callback(contact)));
    }
}

extern "C" fn physics_contact_delegate_did_end(context: *mut c_void, contact_handle: *mut c_void) {
    let context = unsafe { &*(context.cast::<PhysicsContactDelegateContext>()) };
    let Some(callback) = &context.did_end else {
        if !contact_handle.is_null() {
            unsafe { ffi::sk_release(contact_handle) };
        }
        return;
    };
    if let Some(contact) = unsafe { PhysicsContact::from_raw(contact_handle) } {
        let _ = catch_unwind(AssertUnwindSafe(|| callback(contact)));
    }
}

extern "C" fn physics_contact_delegate_release(context: *mut c_void) {
    if context.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(
            context.cast::<PhysicsContactDelegateContext>(),
        ));
    }
}

impl PhysicsContact {
    #[must_use]
    pub fn body_a(&self) -> Option<PhysicsBody> {
        unsafe { PhysicsBody::from_raw(ffi::sk_physics_contact_get_body_a(self.ptr)) }
    }

    #[must_use]
    pub fn body_b(&self) -> Option<PhysicsBody> {
        unsafe { PhysicsBody::from_raw(ffi::sk_physics_contact_get_body_b(self.ptr)) }
    }

    #[must_use]
    pub fn contact_point(&self) -> CGPoint {
        CGPoint::new(
            unsafe { ffi::sk_physics_contact_get_contact_point_x(self.ptr) },
            unsafe { ffi::sk_physics_contact_get_contact_point_y(self.ptr) },
        )
    }

    #[must_use]
    pub fn contact_normal(&self) -> CGVector {
        CGVector::new(
            unsafe { ffi::sk_physics_contact_get_contact_normal_dx(self.ptr) },
            unsafe { ffi::sk_physics_contact_get_contact_normal_dy(self.ptr) },
        )
    }

    #[must_use]
    pub fn collision_impulse(&self) -> f64 {
        unsafe { ffi::sk_physics_contact_get_collision_impulse(self.ptr) }
    }
}

impl PhysicsContactDelegate {
    #[must_use]
    pub fn new() -> Option<Self> {
        Self::from_callbacks(None::<fn(PhysicsContact)>, None::<fn(PhysicsContact)>)
    }

    #[must_use]
    pub fn from_callbacks<FB, FE>(did_begin: Option<FB>, did_end: Option<FE>) -> Option<Self>
    where
        FB: Fn(PhysicsContact) + Send + Sync + 'static,
        FE: Fn(PhysicsContact) + Send + Sync + 'static,
    {
        let context = Box::new(PhysicsContactDelegateContext {
            did_begin: did_begin.map(|callback| Box::new(callback) as ContactCallback),
            did_end: did_end.map(|callback| Box::new(callback) as ContactCallback),
        });
        unsafe {
            Self::from_raw(ffi::sk_physics_contact_delegate_new(
                Box::into_raw(context).cast(),
                Some(physics_contact_delegate_did_begin),
                Some(physics_contact_delegate_did_end),
                Some(physics_contact_delegate_release),
            ))
        }
    }
}
