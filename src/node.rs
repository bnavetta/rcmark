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

/// An element of a CommonMark document. This includes headers, paragraphs,
/// links, and various other types. Nodes have a variety of properties depending
/// on their type, all of which are exposed here. However, attempting to access
/// a property that does not apply to the node's type will result in a `panic`.
/// Setters return `true` or `false` to indicate whether the node was updated
/// successfully.
pub struct Node { raw: *mut raw::cmark_node,owned: bool,
}

impl Node {
    /// Wrap a raw `cmark_node` in a `Node` instance. Since libcmark uses
    /// `cmark_node` pointers for both owned and unowned references, ownership
    /// must be manually specified. This is to ensure that, for example, the
    /// return value of `node.parent()` is not freed but the return value of
    /// `Node::new` is.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// unsafe {
    ///     let my_node = Node::from_raw(some_raw_node, false);
    /// }
    /// ```
    pub unsafe fn from_raw(raw: *mut raw::cmark_node, owned: bool) -> Node {
        Node {
            raw: raw,
            owned: owned,
        }
    }

    /// Create a new node of type `node_type`. Other type-specific properties
    /// will not be initialized.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcmark::{Node, NodeType};
    ///
    /// let my_node = Node::new(NodeType::Paragraph);
    /// ```
    pub fn new(node_type: NodeType) -> Node {
        unsafe {
            Node::from_raw(raw::cmark_node_new(node_type.raw()), true)
        }
    }

    /// Get the next node in sequence after this one, or `None` if there is not
    /// a subsequent node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcmark::{Node, parse_document, DEFAULT};
    ///
    /// let my_doc = parse_document("*Hello*", DEFAULT);
    /// assert_eq!(my_doc.next(), None);
    /// ```
    pub fn next(&self) -> Option<Node> {
        unsafe {
            Binding::from_raw(raw::cmark_node_next(self.raw))
        }
    }

    /// Get the previous node in the sequence from this one, or `None` if there
    /// is no prior node.
    pub fn previous(&self) -> Option<Node> {
        unsafe {
            Binding::from_raw(raw::cmark_node_previous(self.raw))
        }
    }

    /// Get the parent node that contains this node, or `None` if this node does
    /// not have a parent.
    pub fn parent(&self) -> Option<Node> {
        unsafe {
            Binding::from_raw(raw::cmark_node_parent(self.raw))
        }
    }

    /// Get the first child of this node, or `None` if this node has no children.
    pub fn first_child(&self) -> Option<Node> {
        unsafe {
            Binding::from_raw(raw::cmark_node_first_child(self.raw))
        }
    }

    /// Get the last child of this node, or `None` if this node has no children.
    pub fn last_child(&self) -> Option<Node> {
        unsafe {
            Binding::from_raw(raw::cmark_node_last_child(self.raw))
        }
    }

    /// Create a new iterator over this node and its children.
    pub fn iter(&self) -> NodeIterator {
        NodeIterator::new(self)
    }

    /// Get the type of this node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcmark::{Node, NodeType};
    ///
    /// let my_node = Node::new(NodeType::Strong);
    /// assert_eq!(NodeType::Strong, my_node.node_type());
    /// ```
    pub fn node_type(&self) -> NodeType {
        unsafe {
            Binding::from_raw(raw::cmark_node_get_type(self.raw))
        }
    }

    /// Get a string representation of the type of this node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcmark::{Node, NodeType};
    ///
    /// let my_node = Node::new(NodeType::List);
    /// assert_eq!(my_node.type_string(), "list");
    /// ```
    node_getter!(type_string : &str);

    /// Get the string contents of this node.
    ///
    /// # Panics
    /// If the node type is not `Code`, `Text`, `Html`, or `InnerHtml`.
    node_getter!(literal : &str => NodeType::CodeBlock,
                 NodeType::Code,
                 NodeType::Text,
                 NodeType::Html,
                 NodeType::InlineHtml);

    /// Set the string contents of this node.
    ///
    /// # Panics
    /// If the node type is not `Code`, `Text`, `Html`, or `InnerHtml`.
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
