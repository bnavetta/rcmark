//!# libcmark bindings for Rust
//!
//! This library contains bindings to the [libcmark][1] C library, which is the
//! C reference implementation of the [CommonMark][2] standard. This binding
//! does no additional parsing work beyond that of the underlying library, so it
//! ought to be as accurate.
//!
//! [1]: https://github.com/jgm/cmark
//! [2]: http://commonmark.org/
//!
//!## Nodes
//!
//! The `Node` is the core abstraction in rcmark. Nodes can be built up
//! programmatically or by parsing CommonMark source. Nodes all have a type and
//! may have parent, child, and sibling nodes. Depending on the node type, a
//! variety of properties are available. If a property is not applicable to a
//! given node type, then attempting to access it will panic.
//!
//!```
//! use rcmark::{Node, NodeType, ListType};
//!
//! let mut root = Node::new(NodeType::Document);
//!
//! let mut heading = Node::new(NodeType::Header);
//! heading.set_header_level(1);
//!
//! let mut heading_text = Node::new(NodeType::Text);
//! heading_text.set_literal("Hello, World!");
//!
//! heading.prepend_child(&mut heading_text);
//! root.prepend_child(&mut heading);
//!```
//!
//!## Parsing a Document
//!
//! Parsing can be done through either a `Parser` instance or
//! the all-in-one `parse_document` function.
//!
//!```
//! use rcmark::{Parser, parse_document, DEFAULT, NORMALIZE};
//!
//! let doc = parse_document("**Hello**, `World!`", DEFAULT);
//! assert_eq!(doc.start_line(), 1);
//!
//! let mut parser = Parser::new(NORMALIZE);
//! parser.feed("# Hello, World!");
//! let doc2 = parser.finish();
//!```
//!
//!## Rendering a Document
//!
//! Rendering could be done manually, but libcmark also provides
//! functions to render to XML, HTML, man pages, and CommonMark.
//!
//!```
//! let doc = rcmark::parse_document("# Hello", rcmark::DEFAULT);
//!
//! assert_eq!(rcmark::render_xml(&doc, rcmark::DEFAULT),
//!            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
//!             <!DOCTYPE CommonMark SYSTEM \"CommonMark.dtd\">\n\
//!             <document>\n  \
//!               <header level=\"1\">\n    \
//!                 <text>Hello</text>\n  \
//!               </header>\n\
//!             </document>\n");
//! assert_eq!(rcmark::render_html(&doc, rcmark::DEFAULT),
//!            "<h1>Hello</h1>\n");
//! assert_eq!(rcmark::render_man(&doc, rcmark::DEFAULT),
//!            ".SH\nHello\n");
//! assert_eq!(rcmark::render_commonmark(&doc, rcmark::DEFAULT, 2),
//!            "# Hello\n");
//!```
#![feature(concat_idents)]
#![feature(trace_macros)]
#[deny(missing_docs)]

extern crate libc;
extern crate libcmark_sys as raw;
#[macro_use] extern crate bitflags;

pub use node::Node;
pub use iter::NodeIterator;
pub use parser::{Parser, parse_document};
pub use render::{render_xml, render_html, render_man, render_commonmark};

use util::Binding;

use std::ffi::CStr;
use std::str;

mod node;
mod iter;
mod parser;
mod render;
mod util;

/// The types of nodes that make up a CommonMark document.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum NodeType {
    /// Error status
    None,
    Document,
    BlockQuote,
    List,
    Item,
    CodeBlock,
    Html,
    Paragraph,
    Header,
    Hrule,
    Text,
    SoftBreak,
    LineBreak,
    Code,
    InlineHtml,
    Emph,
    Strong,
    Link,
    Image,
}

impl Binding for NodeType {
    type Raw = raw::cmark_node_type;

    /// Obtain a `NodeType` from the corresponding raw `cmark_node_type` enum value.
    unsafe fn from_raw(raw: raw::cmark_node_type) -> NodeType {
        match raw {
            raw::CMARK_NODE_NONE => NodeType::None,
            raw::CMARK_NODE_DOCUMENT => NodeType::Document,
            raw::CMARK_NODE_BLOCK_QUOTE => NodeType::BlockQuote,
            raw::CMARK_NODE_LIST => NodeType::List,
            raw::CMARK_NODE_ITEM => NodeType::Item,
            raw::CMARK_NODE_CODE_BLOCK => NodeType::CodeBlock,
            raw::CMARK_NODE_HTML => NodeType::Html,
            raw::CMARK_NODE_PARAGRAPH => NodeType::Paragraph,
            raw::CMARK_NODE_HEADER => NodeType::Header,
            raw::CMARK_NODE_HRULE => NodeType::Hrule,
            raw::CMARK_NODE_TEXT => NodeType::Text,
            raw::CMARK_NODE_SOFTBREAK => NodeType::SoftBreak,
            raw::CMARK_NODE_LINEBREAK => NodeType::LineBreak,
            raw::CMARK_NODE_CODE => NodeType::Code,
            raw::CMARK_NODE_INLINE_HTML => NodeType::InlineHtml,
            raw::CMARK_NODE_EMPH => NodeType::Emph,
            raw::CMARK_NODE_STRONG => NodeType::Strong,
            raw::CMARK_NODE_LINK => NodeType::Link,
            raw::CMARK_NODE_IMAGE => NodeType::Image,
        }
    }

    /// Obtain the raw `cmark_node_type` enum value from a `NodeType`.
    fn raw(&self) -> raw::cmark_node_type {
        match *self {
            NodeType::None => raw::CMARK_NODE_NONE,
            NodeType::Document => raw::CMARK_NODE_DOCUMENT,
            NodeType::BlockQuote => raw::CMARK_NODE_BLOCK_QUOTE,
            NodeType::List => raw::CMARK_NODE_LIST,
            NodeType::Item => raw::CMARK_NODE_ITEM,
            NodeType::CodeBlock => raw::CMARK_NODE_CODE_BLOCK,
            NodeType::Html => raw::CMARK_NODE_HTML,
            NodeType::Paragraph => raw::CMARK_NODE_PARAGRAPH,
            NodeType::Header => raw::CMARK_NODE_HEADER,
            NodeType::Hrule => raw::CMARK_NODE_HRULE,
            NodeType::Text => raw::CMARK_NODE_TEXT,
            NodeType::SoftBreak => raw::CMARK_NODE_SOFTBREAK,
            NodeType::LineBreak => raw::CMARK_NODE_LINEBREAK,
            NodeType::Code => raw::CMARK_NODE_CODE,
            NodeType::InlineHtml => raw::CMARK_NODE_INLINE_HTML,
            NodeType::Emph => raw::CMARK_NODE_EMPH,
            NodeType::Strong => raw::CMARK_NODE_STRONG,
            NodeType::Link => raw::CMARK_NODE_LINK,
            NodeType::Image => raw::CMARK_NODE_IMAGE,
        }
    }
}

/// The type of CommonMark list.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ListType {
    NoList,
    Bullet,
    Ordered
}

impl Binding for ListType {
    type Raw = raw::cmark_list_type;

    unsafe fn from_raw(raw: raw::cmark_list_type) -> ListType {
        match raw {
            raw::CMARK_NO_LIST => ListType::NoList,
            raw::CMARK_BULLET_LIST => ListType::Bullet,
            raw::CMARK_ORDERED_LIST => ListType::Ordered,
        }
    }

    fn raw(&self) -> raw::cmark_list_type {
        match *self {
            ListType::NoList => raw::CMARK_NO_LIST,
            ListType::Bullet => raw::CMARK_BULLET_LIST,
            ListType::Ordered => raw::CMARK_ORDERED_LIST,
        }
    }
}

// The type of list delimiter in an ordered list.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum DelimType {
    NoDelim,
    /// Numbers are written as `1.`
    Period,
    /// Numbers are written as `1)`
    Paren
}

impl Binding for DelimType {
    type Raw = raw::cmark_delim_type;

    unsafe fn from_raw(raw: raw::cmark_delim_type) -> DelimType {
        match raw {
            raw::CMARK_NO_DELIM => DelimType::NoDelim,
            raw::CMARK_PERIOD_DELIM => DelimType::Period,
            raw::CMARK_PAREN_DELIM => DelimType::Paren,
        }
    }

    fn raw(&self) -> raw::cmark_delim_type {
        match *self {
            DelimType::NoDelim => raw::CMARK_NO_DELIM,
            DelimType::Period => raw::CMARK_PERIOD_DELIM,
            DelimType::Paren => raw::CMARK_PAREN_DELIM,
        }
    }
}

/// The event types that may be produced by a node iterator.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum EventType {
    None,
    Done,
    Enter,
    Exit
}

impl Binding for EventType {
    type Raw = raw::cmark_event_type;

    unsafe fn from_raw(raw: raw::cmark_event_type) -> EventType {
        match raw {
            raw::CMARK_EVENT_NONE => EventType::None,
            raw::CMARK_EVENT_DONE => EventType::Done,
            raw::CMARK_EVENT_ENTER => EventType::Enter,
            raw::CMARK_EVENT_EXIT => EventType::Exit,
        }
    }

    fn raw(&self) -> raw::cmark_event_type {
        match *self {
            EventType::None => raw::CMARK_EVENT_NONE,
            EventType::Done => raw::CMARK_EVENT_DONE,
            EventType::Enter => raw::CMARK_EVENT_ENTER,
            EventType::Exit => raw::CMARK_EVENT_EXIT,
        }
    }
}

/// Options for parsing and rendering a node tree.
bitflags! {
    flags CmarkOptions: i32 {
        #[doc="Default writer options"]
        const DEFAULT = raw::CMARK_OPT_DEFAULT as i32,
        #[doc="Include a `data-sourcepos` attribute on block elements"]
        const SOURCEPOS = raw::CMARK_OPT_SOURCEPOS as i32,
        #[doc="Render `softbreak` elements as hard line breaks"]
        const HARDBREAKS = raw::CMARK_OPT_HARDBREAKS as i32,
        #[doc="Normalize the tree by consolidating adjacent text nodes"]
        const NORMALIZE = raw::CMARK_OPT_NORMALIZE as i32,
        #[doc="Convert straight quotes to curly quotes, `---` to `—`, and `--` to `–`"]
        const SMART = raw::CMARK_OPT_SMART as i32,
    }
}

impl Binding for CmarkOptions {
    type Raw = libc::c_int;

    unsafe fn from_raw(raw: libc::c_int) -> CmarkOptions {
        CmarkOptions::from_bits_truncate(Binding::from_raw(raw))
    }

    fn raw(&self) -> libc::c_int {
        self.bits as libc::c_int
    }
}

pub fn cmark_version() -> i32 {
    raw::cmark_version as i32
}

pub fn version<'a>() -> &'a str {
    unsafe {
        str::from_utf8(
            CStr::from_ptr(raw::cmark_version_string).to_bytes()).unwrap()
    }
}
