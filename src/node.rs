use std::marker::PhantomData;

use {raw, NodeType, ListType, DelimType};

pub struct Node<'node> {
    raw: *mut raw::cmark_node,
    phantom: PhantomData<&'node Node<'node>>
}

impl<'node> Node<'node> {
    pub unsafe fn from_raw(raw: *mut raw::cmark_node) -> Node<'node> {
        Node {
            raw: raw,
        }
    }

    pub fn raw(&self) -> *mut raw::cmark_node { self.raw }

    pub fn new(node_type: NodeType) -> Node<'node> {
        unsafe {
            Node::from_raw(raw::cmark_node_new(node_type.raw()))
        }
    }

    pub fn next(&self) -> Option<Node<'node>> {
        unsafe {
            let next_ptr = raw::cmark_node_next(self.raw);
            if next_ptr.is_null() {
                None
            }
            else {
                Some(Node::from_raw(next_ptr))
            }
        }
    }

    pub fn previous(&self) -> Option<Node<'node>> {
        unsafe {
            let ptr = raw::cmark_node_previous(self.raw);
            if ptr.is_null() {
                None
            }
            else {
                Some(Node::from_raw(ptr))
            }
        }
    }

    pub fn parent(&self) -> Option<Node<'node>> {
        unsafe {
            let ptr = raw::cmark_node_parent(self.raw);
            if ptr.is_null() {
                None
            }
            else {
                Some(Node::from_raw(ptr))
            }
        }
    }

    pub fn first_child(&self) -> Option<Node<'node>> {
        unsafe {
            let ptr = raw::cmark_node_first_child(self.raw);
            if ptr.is_null() {
                None
            }
            else {
                Some(Node::from_raw(ptr))
            }
        }
    }

    pub fn last_child(&self) -> Option<Node<'node>> {
        unsafe {
            let ptr = raw::cmark_node_last_child(self.raw);
            if ptr.is_null() {
                None
            }
            else {
                Some(Node::from_raw(ptr))
            }
        }
    }
}

impl<'node> Drop for Node<'node> {
    fn drop(&mut self) {
        unsafe { raw::cmark_node_free(self.raw) }
    }
}
