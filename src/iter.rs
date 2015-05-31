use {raw, EventType, Node};

use std::marker::PhantomData;

pub struct NodeIterator<'iter> {
    raw: *mut raw::cmark_iter,
    phantom: PhantomData<&'iter Node<'iter>>
}

impl<'iter> NodeIterator<'iter> {
    pub fn from_raw(raw: *mut raw::cmark_iter) -> NodeIterator<'iter> {
        NodeIterator {
            raw: raw,
            phantom: PhantomData,
        }
    }

    pub fn raw(&self) -> *mut raw::cmark_iter { self.raw }

    pub fn new(root: &Node<'iter>) -> NodeIterator<'iter> {
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

    pub fn node(&self) -> &'iter Node {
        unsafe {
            let node_ptr: &'iter mut raw::cmark_node = raw::cmark_iter_get_node(self.raw);
            &Node::from_raw(node_ptr);
            // &Node::from_raw(raw::cmark_iter_get_node(self.raw))
        }
    }

    pub fn event_type(&self) -> EventType {
        EventType::from_raw(unsafe {
            raw::cmark_iter_get_event_type(self.raw)
        })
    }

    pub fn root(&self) -> &'iter Node {
        unsafe {
            &Node::from_raw(raw::cmark_iter_get_root(self.raw))
        }
    }

    pub fn iter(&'iter self) -> Iter<'iter> {
        Iter {
            node_iter: self,
        }
    }
}

impl<'iter> Drop for NodeIterator<'iter> {
    fn drop(&mut self) {
        unsafe {
            raw::cmark_iter_free(self.raw)
        }
    }
}

impl<'iter> IntoIterator for &'iter NodeIterator<'iter> {
    type Item = (EventType, &'iter Node<'iter>);
    type IntoIter = Iter<'iter>;

    fn into_iter(self) -> Iter<'iter> {
        self.iter()
    }
}

// impl<'a> IntoIterator for &'a mut NodeIterator {
//     type Item = (EventType, &'a mut Node);
//     type IntoIter = IterMut<'a>;

//     fn into_iter(self) -> IterMut<'a> {
//         self.iter_mut()
//     }
// }

pub struct Iter<'iter> {
    node_iter: &'iter NodeIterator<'iter>,
}

impl<'iter> Iterator for Iter<'iter> {
    type Item = (EventType, &'iter Node<'iter>);

    fn next(&mut self) -> Option<(EventType, &'iter Node)> {
        let next_event_raw = unsafe { raw::cmark_iter_next(self.node_iter.raw) };
        if next_event_raw == raw::CMARK_EVENT_NONE {
            None
        }
        else {
            let next_event = EventType::from_raw(next_event_raw);
            let next_node = self.node_iter.node();
            Some((next_event, &next_node))
        }
    }
}

// impl Iterator for NodeIterator {
//     type Item = (EventType, &mut Node);

//     pub fn next<'a>(&'a mut self) -> Option<(EventType, &'a mut Node)> {
//         let next_event_raw = unsafe { raw::cmark_iter_next(self.raw) };
//         if next_event_raw == raw::CMARK_EVENT_NONE {
//             None
//         }
//         else {
//             let next_event = EventType::from_raw(next_event_raw);
//             let next_node = self.node();
//             Some((next_event, next_node))
//         }
//     }
// }
