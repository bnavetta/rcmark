use {raw, EventType, Node};
use util::Binding;

pub struct NodeIterator {
    raw: *mut raw::cmark_iter,
}

impl NodeIterator {
    pub fn from_raw(raw: *mut raw::cmark_iter) -> NodeIterator {
        NodeIterator {
            raw: raw,
        }
    }

    pub fn raw(&self) -> *mut raw::cmark_iter { self.raw }

    pub fn new(root: &Node) -> NodeIterator {
        let raw_iter = unsafe {
            raw::cmark_iter_new(root.raw())
        };
        NodeIterator::from_raw(raw_iter)
    }

    pub fn reset(&mut self, current: &Node, event_type: EventType) {
        unsafe {
            raw::cmark_iter_reset(self.raw, current.raw(), event_type.raw())
        }
    }

    pub fn node(&self) -> Node {
        unsafe {
            Node::from_raw(raw::cmark_iter_get_node(self.raw), false)
        }
    }

    pub fn event_type(&self) -> EventType {
        unsafe {
            Binding::from_raw(raw::cmark_iter_get_event_type(self.raw))
        }
    }

    pub fn root(&self) -> Node {
        unsafe {
            Node::from_raw(raw::cmark_iter_get_root(self.raw), false)
        }
    }
}

impl Drop for NodeIterator {
    fn drop(&mut self) {
        unsafe {
            raw::cmark_iter_free(self.raw)
        }
    }
}

impl Iterator for NodeIterator {
    type Item = (EventType, Node);

    fn next(&mut self) -> Option<(EventType, Node)> {
        let next_event_raw = unsafe { raw::cmark_iter_next(self.raw) };
        if next_event_raw == raw::CMARK_EVENT_NONE {
            None
        }
        else {
            let next_event = unsafe { Binding::from_raw(next_event_raw) };
            let next_node = self.node();
            Some((next_event, next_node))
        }
    }
}
