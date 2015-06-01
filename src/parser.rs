
use {raw, Node, CmarkOptions};

use std::ffi::CString;
use libc;

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

    pub fn new(options: CmarkOptions) -> Parser {
        unsafe {
            Parser::from_raw(raw::cmark_parser_new(options.raw()))
        }
    }

    pub fn feed(&mut self, data: &str) {
        unsafe {
            let cstr = CString::new(data).unwrap();
            raw::cmark_parser_feed(self.raw, cstr.as_ptr(), cstr.to_bytes().len() as libc::size_t)
        }
    }

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
        assert!(text.literal().unwrap() == "Hi");
    }
}
