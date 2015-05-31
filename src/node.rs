use {raw, NodeType, ListType, DelimType, NodeIterator};

// The representation of nodes is made somewhat difficult by the fact that
// libcmark uses cmark_node* for both owned and unowned references. For example,
// both cmark_node_new and cmark_node_parent return a cmark_node*, but only
// the first should eventually be freed by the caller. Moreover, the Rust Node
// struct cannot be returned as a reference, because it is created as needed
// (the pointer it wraps may be owned elsewhere, but it is not). Ideally, this
// would map to Rust lifetimes, probably with the use of the PhantomData marker,
// but for the sake of simplicity, the Node struct simply keeps track of whether
// it owns its raw node pointer.

pub struct Node { raw: *mut raw::cmark_node,owned: bool,
}

impl Node {
    pub unsafe fn from_raw(raw: *mut raw::cmark_node, owned: bool) -> Node {
        Node {
            raw: raw,
            owned: owned,
        }
    }

    pub fn raw(&self) -> *mut raw::cmark_node { self.raw }

    pub fn new(node_type: NodeType) -> Node {
        unsafe {
            Node::from_raw(raw::cmark_node_new(node_type.raw()), true)
        }
    }

    pub fn next(&self) -> Option<Node> {
        unsafe {
            let next_ptr = raw::cmark_node_next(self.raw);
            if next_ptr.is_null() {
                None
            }
            else {
                Some(Node::from_raw(next_ptr, false))
            }
        }
    }

    pub fn previous(&self) -> Option<Node> {
        unsafe {
            let ptr = raw::cmark_node_previous(self.raw);
            if ptr.is_null() {
                None
            }
            else {
                Some(Node::from_raw(ptr, false))
            }
        }
    }

    pub fn parent(&self) -> Option<Node> {
        unsafe {
            let ptr = raw::cmark_node_parent(self.raw);
            if ptr.is_null() {
                None
            }
            else {
                Some(Node::from_raw(ptr, false))
            }
        }
    }

    pub fn first_child(&self) -> Option<Node> {
        unsafe {
            let ptr = raw::cmark_node_first_child(self.raw);
            if ptr.is_null() {
                None
            }
            else {
                Some(Node::from_raw(ptr, false))
            }
        }
    }

    pub fn last_child(&self) -> Option<Node> {
        unsafe {
            let ptr = raw::cmark_node_last_child(self.raw);
            if ptr.is_null() {
                None
            }
            else {
                Some(Node::from_raw(ptr, false))
            }
        }
    }

    pub fn iter(&self) -> NodeIterator {
        NodeIterator::new(self)
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        if self.owned {
            unsafe { raw::cmark_node_free(self.raw) }
        }
    }
}
