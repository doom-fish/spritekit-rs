use apple_cf::cg::CGPoint;

use crate::ffi;
use crate::node::AsNode;
use crate::private::handle_type;

handle_type!(Event);

impl Event {
    #[must_use]
    pub fn mouse_moved(location: CGPoint) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_event_mouse_moved(location.x, location.y)) }
    }

    #[must_use]
    pub fn location_in_node<N: AsNode>(&self, node: &N) -> CGPoint {
        CGPoint::new(
            unsafe { ffi::sk_event_location_in_node_x(self.ptr, node.as_node_ptr()) },
            unsafe { ffi::sk_event_location_in_node_y(self.ptr, node.as_node_ptr()) },
        )
    }
}
