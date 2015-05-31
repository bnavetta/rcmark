extern crate libc;
extern crate libcmark_sys as raw;

pub use node::Node;
pub use iter::NodeIterator;

mod node;
mod iter;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum NodeType {
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

impl NodeType {
    pub fn from_raw(raw_type: raw::cmark_node_type) -> NodeType {
        match raw_type {
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

    pub fn raw(&self) -> raw::cmark_node_type {
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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ListType {
    NoList,
    Bullet,
    Ordered
}

impl ListType {
    pub fn from_raw(raw_type: raw::cmark_list_type) -> ListType {
        match raw_type {
            raw::CMARK_NO_LIST => ListType::NoList,
            raw::CMARK_BULLET_LIST => ListType::Bullet,
            raw::CMARK_ORDERED_LIST => ListType::Ordered,
        }
    }

    pub fn raw(&self) -> raw::cmark_list_type {
        match *self {
            ListType::NoList => raw::CMARK_NO_LIST,
            ListType::Bullet => raw::CMARK_BULLET_LIST,
            ListType::Ordered => raw::CMARK_ORDERED_LIST,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum DelimType {
    NoDelim,
    Period,
    Paren
}

impl DelimType {
    pub fn from_raw(raw_type: raw::cmark_delim_type) -> DelimType {
        match raw_type {
            raw::CMARK_NO_DELIM => DelimType::NoDelim,
            raw::CMARK_PERIOD_DELIM => DelimType::Period,
            raw::CMARK_PAREN_DELIM => DelimType::Paren,
        }
    }

    pub fn raw(&self) -> raw::cmark_delim_type {
        match *self {
            DelimType::NoDelim => raw::CMARK_NO_DELIM,
            DelimType::Period => raw::CMARK_PERIOD_DELIM,
            DelimType::Paren => raw::CMARK_PAREN_DELIM,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum EventType {
    None,
    Done,
    Enter,
    Exit
}

impl EventType {
    pub fn from_raw(raw_type: raw::cmark_event_type) -> EventType {
        match raw_type {
            raw::CMARK_EVENT_NONE => EventType::None,
            raw::CMARK_EVENT_DONE => EventType::Done,
            raw::CMARK_EVENT_ENTER => EventType::Enter,
            raw::CMARK_EVENT_EXIT => EventType::Exit,
        }
    }

    pub fn raw(&self) -> raw::cmark_event_type {
        match *self {
            EventType::None => raw::CMARK_EVENT_NONE,
            EventType::Done => raw::CMARK_EVENT_DONE,
            EventType::Enter => raw::CMARK_EVENT_ENTER,
            EventType::Exit => raw::CMARK_EVENT_EXIT,
        }
    }
}
