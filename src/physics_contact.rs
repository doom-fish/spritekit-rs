use core::ffi::c_void;

use apple_cf::cg::{CGPoint, CGVector};
use doom_fish_utils::callback_context::CallbackContext;

use crate::ffi;
use crate::physics_body::PhysicsBody;
use crate::private::{handle_type, invoke_callback, DelegateObject};

handle_type!(PhysicsContact);

type ContactCallback = Box<dyn Fn(PhysicsContact) + Send + Sync + 'static>;

struct PhysicsContactDelegateContext {
    did_begin: Option<ContactCallback>,
    did_end: Option<ContactCallback>,
}

#[derive(Debug)]
pub struct PhysicsContactDelegate {
    inner: DelegateObject<PhysicsContactDelegateContext>,
}

unsafe fn deliver_contact(
    context: *mut c_void,
    contact_handle: *mut c_void,
    site: &str,
    select: fn(&PhysicsContactDelegateContext) -> Option<&ContactCallback>,
) {
    let Some(contact) = (unsafe { PhysicsContact::from_raw(contact_handle) }) else {
        return;
    };
    unsafe {
        invoke_callback::<PhysicsContactDelegateContext, _>(context, site, |delegate| {
            if let Some(callback) = select(delegate) {
                callback(contact);
            }
        });
    }
}

unsafe extern "C" fn physics_contact_delegate_did_begin(
    context: *mut c_void,
    contact_handle: *mut c_void,
) {
    unsafe {
        deliver_contact(
            context,
            contact_handle,
            "spritekit::PhysicsContactDelegate::did_begin",
            |delegate| delegate.did_begin.as_ref(),
        );
    }
}

unsafe extern "C" fn physics_contact_delegate_did_end(
    context: *mut c_void,
    contact_handle: *mut c_void,
) {
    unsafe {
        deliver_contact(
            context,
            contact_handle,
            "spritekit::PhysicsContactDelegate::did_end",
            |delegate| delegate.did_end.as_ref(),
        );
    }
}

impl PhysicsContact {
    /// Wraps `SKPhysicsContact`.
    #[must_use]
    pub fn body_a(&self) -> Option<PhysicsBody> {
        unsafe { PhysicsBody::from_raw(ffi::sk_physics_contact_get_body_a(self.ptr)) }
    }

    /// Wraps `SKPhysicsContact`.
    #[must_use]
    pub fn body_b(&self) -> Option<PhysicsBody> {
        unsafe { PhysicsBody::from_raw(ffi::sk_physics_contact_get_body_b(self.ptr)) }
    }

    /// Wraps `SKPhysicsContact`.
    #[must_use]
    pub fn contact_point(&self) -> CGPoint {
        CGPoint::new(
            unsafe { ffi::sk_physics_contact_get_contact_point_x(self.ptr) },
            unsafe { ffi::sk_physics_contact_get_contact_point_y(self.ptr) },
        )
    }

    /// Wraps `SKPhysicsContact`.
    #[must_use]
    pub fn contact_normal(&self) -> CGVector {
        CGVector::new(
            unsafe { ffi::sk_physics_contact_get_contact_normal_dx(self.ptr) },
            unsafe { ffi::sk_physics_contact_get_contact_normal_dy(self.ptr) },
        )
    }

    /// Wraps `SKPhysicsContact`.
    #[must_use]
    pub fn collision_impulse(&self) -> f64 {
        unsafe { ffi::sk_physics_contact_get_collision_impulse(self.ptr) }
    }
}

impl PhysicsContactDelegate {
    /// Wraps `SKPhysicsContactDelegate`.
    #[must_use]
    pub fn new() -> Option<Self> {
        Self::from_callbacks(None::<fn(PhysicsContact)>, None::<fn(PhysicsContact)>)
    }

    /// Wraps `SKPhysicsContactDelegate`.
    #[must_use]
    pub fn from_callbacks<FB, FE>(did_begin: Option<FB>, did_end: Option<FE>) -> Option<Self>
    where
        FB: Fn(PhysicsContact) + Send + Sync + 'static,
        FE: Fn(PhysicsContact) + Send + Sync + 'static,
    {
        let context = PhysicsContactDelegateContext {
            did_begin: did_begin.map(|callback| Box::new(callback) as ContactCallback),
            did_end: did_end.map(|callback| Box::new(callback) as ContactCallback),
        };
        DelegateObject::new(context, |context| unsafe {
            ffi::sk_physics_contact_delegate_new(
                context,
                Some(physics_contact_delegate_did_begin),
                Some(physics_contact_delegate_did_end),
                Some(CallbackContext::<PhysicsContactDelegateContext>::RELEASE),
            )
        })
        .map(|inner| Self { inner })
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.inner.as_ptr()
    }
}
