use {raw, NodeType, ListType, DelimType, NodeIterator};

use std::ffi::{CStr, CString};
use std::str;
use libc;

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

    pub fn node_type(&self) -> NodeType {
        NodeType::from_raw(unsafe {
            raw::cmark_node_get_type(self.raw)
        })
    }

    pub fn node_type_string(&self) -> &str {
        unsafe {
            let c_str =
                CStr::from_ptr(raw::cmark_node_get_type_string(self.raw));
            str::from_utf8(c_str.to_bytes()).unwrap()
        }
    }

    pub fn literal(&self) -> Option<&str> {
        unsafe {
            from_string_ptr(raw::cmark_node_get_literal(self.raw))
        }
    }

    pub fn set_literal(&mut self, content: &str) -> bool {
        unsafe {
            let cstr = CString::new(content).unwrap();
            to_bool(raw::cmark_node_set_literal(self.raw, cstr.as_ptr()))
        }
    }

    pub fn header_level(&self) -> i32 {
        unsafe {
            raw::cmark_node_get_header_level(self.raw) as i32
        }
    }

    pub fn set_header_level(&mut self, level: i32) -> bool {
        unsafe {
            to_bool(raw::cmark_node_set_header_level(self.raw, level as libc::c_int))
        }
    }

    pub fn list_type(&self) -> ListType {
        unsafe {
           ListType::from_raw(raw::cmark_node_get_list_type(self.raw))
        }
    }

    pub fn set_list_type(&mut self, list_type: ListType) -> bool {
        unsafe {
            to_bool(raw::cmark_node_set_list_type(self.raw, list_type.raw()))
        }
    }

    pub fn delim_type(&self) -> DelimType {
        unsafe {
            DelimType::from_raw(raw::cmark_node_get_delim_type(self.raw))
        }
    }

    pub fn set_delim_type(&mut self, delim_type: DelimType) -> bool {
        unsafe {
            to_bool(raw::cmark_node_set_delim_type(self.raw, delim_type.raw()))
        }
    }

    pub fn list_start(&self) -> i32 {
        unsafe {
            raw::cmark_node_get_list_start(self.raw) as i32
        }
    }

    pub fn set_list_start(&mut self, start: i32) -> bool {
        unsafe {
            to_bool(raw::cmark_node_set_list_start(self.raw, start as libc::c_int))
        }
    }

    pub fn list_tight(&self) -> bool {
        unsafe {
            to_bool(raw::cmark_node_get_list_tight(self.raw))
        }
    }

    pub fn set_list_tight(&mut self, tight: bool) -> bool {
        unsafe {
            to_bool(raw::cmark_node_set_list_tight(self.raw, from_bool(tight)))
        }
    }

    pub fn fence_info(&self) -> Option<&str> {
        unsafe {
            from_string_ptr(raw::cmark_node_get_fence_info(self.raw))
        }
    }

    pub fn set_fence_info(&mut self, info: &str) -> bool {
        unsafe {
            let cstr = CString::new(info).unwrap();
            to_bool(raw::cmark_node_set_fence_info(self.raw, cstr.as_ptr()))
        }
    }

    pub fn url(&self) -> Option<&str> {
        unsafe {
            from_string_ptr(raw::cmark_node_get_url(self.raw))
        }
    }

    pub fn set_url(&mut self, url: &str) -> bool {
        unsafe {
            let cstr = CString::new(url).unwrap();
            to_bool(raw::cmark_node_set_url(self.raw, cstr.as_ptr()))
        }
    }

    pub fn title(&self) -> Option<&str> {
        unsafe {
            from_string_ptr(raw::cmark_node_get_title(self.raw))
        }
    }

    pub fn set_title(&mut self, title: &str) -> bool {
        unsafe {
            let cstr = CString::new(title).unwrap();
            to_bool(raw::cmark_node_set_title(self.raw, cstr.as_ptr()))
        }
    }

    pub fn start_line(&self) -> i32 {
        unsafe {
            raw::cmark_node_get_start_line(self.raw) as i32
        }
    }

    pub fn start_column(&self) -> i32 {
        unsafe {
            raw::cmark_node_get_start_column(self.raw) as i32
        }
    }

    pub fn end_column(&self) -> i32 {
        unsafe {
            raw::cmark_node_get_end_column(self.raw) as i32
        }
    }

    pub fn end_line(&self) -> i32 {
        unsafe {
            raw::cmark_node_get_end_line(self.raw) as i32
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        if self.owned {
            unsafe { raw::cmark_node_free(self.raw) }
        }
    }
}

fn to_bool(val: libc::c_int) -> bool {
    match val {
        0 => false,
        1 => true,
        _ => panic!("Cannot convert to bool: {}", val)
    }
}

fn from_bool(val: bool) -> libc::c_int {
    match val {
        true => 1 as libc::c_int,
        false => 0 as libc::c_int,
    }
}

unsafe fn from_string_ptr<'a>(ptr: *const libc::c_char) -> Option<&'a str> {
    if ptr.is_null() {
        None
    }
    else {
        let c_str = CStr::from_ptr(ptr);
        str::from_utf8(c_str.to_bytes()).ok()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::NodeType;

    #[test]
    fn test_node_type() {
        let node = Node::new(NodeType::List);
        assert!(node.node_type() == NodeType::List);
        assert!(node.node_type_string() == "list");
    }
}
