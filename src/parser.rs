
use {raw, Node, CmarkOptions};
use util::Binding;

use std::ffi::CString;
use libc;

/// Parsers can be streamed data to parse into a CommonMark AST.
pub struct Parser {
    raw: *mut raw::cmark_parser,
}

impl Parser {
    pub unsafe fn from_raw(raw: *mut raw::cmark_parser) -> Parser {
        Parser {
            raw: raw,
        }
    }

    pub fn raw(&self) -> *mut raw::cmark_parser { self.raw }

    /// Create a new parser with the given options.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcmark::{Parser, NORMALIZE, SMART};
    ///
    /// let mut parser = Parser::new(NORMALIZE | SMART);
    /// ```
    pub fn new(options: CmarkOptions) -> Parser {
        unsafe {
            Parser::from_raw(raw::cmark_parser_new(options.raw()))
        }
    }

    /// Feed additional data into the parser.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcmark::{Parser, DEFAULT};
    ///
    /// let mut parser = Parser::new(DEFAULT);
    /// parser.feed("My *name* is **Inigo Montoya**");
    /// ```
    pub fn feed(&mut self, data: &str) {
        unsafe {
            let cstr = CString::new(data).unwrap();
            raw::cmark_parser_feed(self.raw, cstr.as_ptr(), cstr.to_bytes().len() as libc::size_t)
        }
    }

    /// Finish parsing and return the resulting node tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcmark::{Parser, NodeType, DEFAULT};
    ///
    /// let mut parser = Parser::new(DEFAULT);
    /// parser.feed("*Words*");
    /// let doc = parser.finish();
    /// assert_eq!(doc.node_type(), NodeType::Document);
    /// ```
    pub fn finish(&mut self) -> Node {
        unsafe {
            Node::from_raw(raw::cmark_parser_finish(self.raw), true)
        }
    }
}

impl Drop for Parser {
    fn drop(&mut self) {
        unsafe {
            raw::cmark_parser_free(self.raw)
        }
    }
}

// TODO: maybe implement appropriate Write traits

/// Parse a document into a CommonMark AST.
///
/// # Examples
///
/// ```
/// use rcmark::{parse_document, DEFAULT, NodeType};
///
/// let doc = parse_document("**Hello, World**", DEFAULT);
/// assert_eq!(doc.node_type(), NodeType::Document);
/// let strong = doc.first_child().unwrap().first_child().unwrap();
/// assert_eq!(strong.node_type(), NodeType::Strong);
/// assert_eq!(strong.first_child().unwrap().literal(), "Hello, World");
/// ```
pub fn parse_document(doc: &str, options: CmarkOptions) -> Node {
    unsafe {
        let c_doc = CString::new(doc).unwrap();
        Node::from_raw(raw::cmark_parse_document(c_doc.as_ptr(), c_doc.to_bytes().len() as libc::size_t, options.raw()), true)
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use super::super::{NodeType, DEFAULT};

    #[test]
    pub fn basic_parse() {
        let markdown = "**Hi**";
        let tree = parse_document(markdown, DEFAULT);

        assert!(tree.node_type() == NodeType::Document);
        let paragraph = tree.first_child().unwrap();
        assert!(paragraph.node_type() == NodeType::Paragraph);
        let strong = paragraph.first_child().unwrap();
        assert!(strong.node_type() == NodeType::Strong);
        let text = strong.first_child().unwrap();
        assert!(text.literal() == "Hi");
    }
}
