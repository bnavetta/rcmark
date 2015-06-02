use {raw, NodeType, ListType, DelimType, NodeIterator};
use util::Binding;

use std::ptr;

macro_rules! node_getter {
    ($prop:ident : $prop_type:ty => $($node_type:path),*) => {
        pub fn $prop(&self) -> $prop_type {
            match self.node_type() {
                $($node_type => (),)*
                _ => panic!("{} nodes do not have property {}", self.type_string(), stringify!($prop)),
            }

            unsafe {
                use raw::*;
                let raw_fn = concat_idents!(cmark_node_get_, $prop);
                Binding::from_raw(raw_fn(self.raw))
            }
        }
    };
    ($prop:ident : $prop_type:ty) => {
        pub fn $prop(&self) -> $prop_type {
            unsafe {
                use raw::*;
                let raw_fn = concat_idents!(cmark_node_get_, $prop);
                Binding::from_raw(raw_fn(self.raw))
            }
        } 
    };
}

macro_rules! node_setter {
    // TODO: need to use setter name instead of property name
    // See https://github.com/rust-lang/rust/issues/12249
    ($setter:ident : $prop_type:ty => $($node_type:path),*) => {
        pub fn $setter(&mut self, value: $prop_type) -> bool {
            unsafe {
                use raw::*;
                let raw_fn = concat_idents!(cmark_node_, $setter);
                Binding::from_raw(raw_fn(self.raw, value.raw()))
            }
        }
    };
}

pub struct Node { raw: *mut raw::cmark_node,owned: bool,
}

impl Node {
     pub unsafe fn from_raw(raw: *mut raw::cmark_node, owned: bool) -> Node {
        Node {
            raw: raw,
            owned: owned,
        }
    }

    pub fn new(node_type: NodeType) -> Node {
        unsafe {
            Node::from_raw(raw::cmark_node_new(node_type.raw()), true)
        }
    }

    pub fn next(&self) -> Option<Node> {
        unsafe {
            Binding::from_raw(raw::cmark_node_next(self.raw))
        }
    }

    pub fn previous(&self) -> Option<Node> {
        unsafe {
            Binding::from_raw(raw::cmark_node_previous(self.raw))
        }
    }

    pub fn parent(&self) -> Option<Node> {
        unsafe {
            Binding::from_raw(raw::cmark_node_parent(self.raw))
        }
    }

    pub fn first_child(&self) -> Option<Node> {
        unsafe {
            Binding::from_raw(raw::cmark_node_first_child(self.raw))
        }
    }

    pub fn last_child(&self) -> Option<Node> {
        unsafe {
            Binding::from_raw(raw::cmark_node_last_child(self.raw))
        }
    }

    pub fn iter(&self) -> NodeIterator {
        NodeIterator::new(self)
    }

    pub fn node_type(&self) -> NodeType {
        unsafe {
            Binding::from_raw(raw::cmark_node_get_type(self.raw))
        }
    }

    node_getter!(type_string : &str);

    node_getter!(literal : &str => NodeType::CodeBlock,
                 NodeType::Code,
                 NodeType::Text,
                 NodeType::Html,
                 NodeType::InlineHtml);
    node_setter!(set_literal : &str => NodeType::CodeBlock,
                 NodeType::Code,
                 NodeType::Text,
                 NodeType::Html,
                 NodeType::InlineHtml);

    node_getter!(header_level : i32 => NodeType::Header);
    node_setter!(set_header_level : i32 => NodeType::Header);

    node_getter!(list_type : ListType => NodeType::List);
    node_setter!(set_list_type : ListType => NodeType::List);

    node_getter!(list_delim : DelimType => NodeType::List);
    node_setter!(set_list_delim : DelimType => NodeType::List);

    node_getter!(list_start : i32 => NodeType::List);
    node_setter!(set_list_start : i32 => NodeType::List);

    node_getter!(list_tight : bool => NodeType::List);
    node_setter!(set_list_tight : bool => NodeType::List);

    node_getter!(fence_info : &str => NodeType::CodeBlock);
    node_setter!(set_fence_info : &str => NodeType::CodeBlock);

    node_getter!(url : &str => NodeType::Link, NodeType::Image);
    node_setter!(set_url : &str => NodeType::Link, NodeType::Image);

    node_getter!(title : &str => NodeType::Link, NodeType::Image);
    node_setter!(set_title : &str => NodeType::Link, NodeType::Image);

    node_getter!(start_line : i32);
    node_getter!(start_column : i32);
    node_getter!(end_column: i32);
    node_getter!(end_line : i32);

    pub fn unlink(&mut self) {
        unsafe {
            raw::cmark_node_unlink(self.raw)
        }
    }

    pub fn insert_before(&mut self, sibling: &mut Node) -> bool {
        unsafe {
            Binding::from_raw(raw::cmark_node_insert_before(self.raw, sibling.raw))
        }
    }

    pub fn insert_after(&mut self, sibling: &mut Node) -> bool {
        unsafe {
            Binding::from_raw(raw::cmark_node_insert_after(self.raw, sibling.raw))
        }
    }

    pub fn prepend_child(&mut self, child: &mut Node) -> bool {
        unsafe {
            Binding::from_raw(raw::cmark_node_prepend_child(self.raw, child.raw))
        }
    }

    pub fn append_child(&mut self, child: &mut Node) -> bool {
        unsafe {
            Binding::from_raw(raw::cmark_node_append_child(self.raw, child.raw))
        }
    }

    pub fn consolidate_text_nodes(&mut self) {
        unsafe {
            raw::cmark_consolidate_text_nodes(self.raw)
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

impl Binding for Node {
    type Raw = *mut raw::cmark_node;

    unsafe fn from_raw(raw: *mut raw::cmark_node) -> Node {
        Node::from_raw(raw, false)
    }

    fn raw(&self) -> *mut raw::cmark_node { self.raw }
}

impl Binding for Option<Node> {
    type Raw = *mut raw::cmark_node;

    unsafe fn from_raw(raw: *mut raw::cmark_node) -> Option<Node> {
        if raw.is_null() {
            None
        }
        else {
            Some(Binding::from_raw(raw))
        }
    }

    fn raw(&self) -> *mut raw::cmark_node {
        match *self {
            Some(ref node) => node.raw(),
            None       => ptr::null_mut(),
        }
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
        assert!(node.type_string() == "list");
    }
}
