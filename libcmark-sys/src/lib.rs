#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_int, c_char, c_void, size_t};

pub use cmark_node_type::*;
pub use cmark_list_type::*;
pub use cmark_delim_type::*;
pub use cmark_event_type::*;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmark_node_type {
    // Error status
    CMARK_NODE_NONE,

    // Block
    CMARK_NODE_DOCUMENT,
    CMARK_NODE_BLOCK_QUOTE,
    CMARK_NODE_LIST,
    CMARK_NODE_ITEM,
    CMARK_NODE_CODE_BLOCK,
	  CMARK_NODE_HTML,
	  CMARK_NODE_PARAGRAPH,
	  CMARK_NODE_HEADER,
	  CMARK_NODE_HRULE,

    // CMARK_NODE_FIRST_BLOCK = CMARK_NODE_DOCUMENT,
    // CMARK_NODE_LAST_BLOCK = CMARK_NODE_HRULE,

    // Inline
    CMARK_NODE_TEXT,
    CMARK_NODE_SOFTBREAK,
    CMARK_NODE_LINEBREAK,
    CMARK_NODE_CODE,
    CMARK_NODE_INLINE_HTML,
    CMARK_NODE_EMPH,
    CMARK_NODE_STRONG,
    CMARK_NODE_LINK,
    CMARK_NODE_IMAGE,

    // CMARK_NODE_FIRST_INLINE = CMARK_NODE_TEXT,
    // CMARK_NODE_LAST_INLINE = CMARK_NODE_IMAGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmark_list_type {
    CMARK_NO_LIST,
    CMARK_BULLET_LIST,
    CMARK_ORDERED_LIST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmark_delim_type {
    CMARK_NO_DELIM,
    CMARK_PERIOD_DELIM,
    CMARK_PAREN_DELIM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmark_event_type {
    CMARK_EVENT_NONE,
    CMARK_EVENT_DONE,
    CMARK_EVENT_ENTER,
    CMARK_EVENT_EXIT,
}

pub enum cmark_node {}
pub enum cmark_parser {}
pub enum cmark_iter {}

pub const CMARK_OPT_DEFAULT: c_int = 0;
pub const CMARK_OPT_SOURCEPOS: c_int = 1;
pub const CMARK_OPT_HARDBREAKS: c_int = 2;
pub const CMARK_OPT_NORMALIZE: c_int = 4;
pub const CMARK_OPT_SMART: c_int = 8;

extern {
    // Basic API
    pub fn cmark_markdown_to_html(text: *const c_char, len: c_int, options: c_int) -> *mut c_char;

    // Node operations
    pub fn cmark_node_new(node_type: cmark_node_type) -> *mut cmark_node;
    pub fn cmark_node_free(node: *mut cmark_node);

    // Tree traversal
    pub fn cmark_node_next(node: *const cmark_node) -> *mut cmark_node;
    pub fn cmark_node_previous(node: *const cmark_node) -> *mut cmark_node;
    pub fn cmark_node_parent(node: *const cmark_node) -> *mut cmark_node;
    pub fn cmark_node_first_child(node: *const cmark_node) -> *mut cmark_node;
    pub fn cmark_node_last_child(node: *const cmark_node) -> *mut cmark_node;

    // Iterator
    pub fn cmark_iter_new(root: *const cmark_node) -> *mut cmark_iter;
    pub fn cmark_iter_free(iter: *mut cmark_iter);
    pub fn cmark_iter_next(iter: *mut cmark_iter) -> cmark_event_type;
    pub fn cmark_iter_get_node(iter: *const cmark_iter) -> *mut cmark_node;
    pub fn cmark_iter_get_event_type(iter: *const cmark_iter) -> cmark_event_type;
    pub fn cmark_iter_get_root(iter: *const cmark_iter) -> *mut cmark_node;
    pub fn cmark_iter_reset(iter: *mut cmark_iter, current: *const cmark_node, event_type: cmark_event_type);

    // Accessors
    pub fn cmark_node_get_user_data(node: *const cmark_node) -> *const c_void;
    pub fn cmark_node_set_user_data(node: *mut cmark_node, user_data: *const c_void) -> c_int;
    pub fn cmark_node_get_type(node: *const cmark_node) -> cmark_node_type;
    pub fn cmark_node_get_type_string(node: *const cmark_node) -> *const c_char;
    pub fn cmark_node_get_literal(node: *const cmark_node) -> *const c_char;
    pub fn cmark_node_set_literal(node: *mut cmark_node, content: *const c_char) -> c_int;
    pub fn cmark_node_get_header_level(node: *const cmark_node) -> c_int;
    pub fn cmark_node_set_header_level(node: *mut cmark_node, level: c_int) -> c_int;
    pub fn cmark_node_get_list_type(node: *const cmark_node) -> cmark_list_type;
    pub fn cmark_node_set_list_type(node: *mut cmark_node, list_type: cmark_list_type) -> c_int;
    pub fn cmark_node_get_list_delim(node: *const cmark_node) -> cmark_delim_type;
    pub fn cmark_node_set_list_delim(node: *mut cmark_node, delim_type: cmark_delim_type) -> c_int;
    pub fn cmark_node_get_list_start(node: *const cmark_node) -> c_int;
    pub fn cmark_node_set_list_start(node: *mut cmark_node, start: c_int) -> c_int;
    pub fn cmark_node_get_list_tight(node: *const cmark_node) -> c_int;
    pub fn cmark_node_set_list_tight(node: *mut cmark_node, tight: c_int) -> c_int;
    pub fn cmark_node_get_fence_info(node: *const cmark_node) -> *const c_char;
    pub fn cmark_node_set_fence_info(node: *mut cmark_node, info: *const c_char) -> c_int;
    pub fn cmark_node_get_url(node: *const cmark_node) -> *const c_char;
    pub fn cmark_node_set_url(node: *mut cmark_node, url: *const c_char) -> c_int;
    pub fn cmark_node_get_title(node: *const cmark_node) -> *const c_char;
    pub fn cmark_node_set_title(node: *mut cmark_node, title: *const c_char) -> c_int;
    pub fn cmark_node_get_start_line(node: *const cmark_node) -> c_int;
    pub fn cmark_node_get_start_column(node: *const cmark_node) -> c_int;
    pub fn cmark_node_get_end_line(node: *const cmark_node) -> c_int;
    pub fn cmark_node_get_end_column(node: *const cmark_node) -> c_int;

    // Tree manipulation
    pub fn cmark_node_unlink(node: *mut cmark_node);
    pub fn cmark_node_insert_before(node: *mut cmark_node, sibling: *mut cmark_node) -> c_int;
    pub fn cmark_node_insert_after(node: *mut cmark_node, sibling: *mut cmark_node) -> c_int;
    pub fn cmark_node_prepend_child(node: *mut cmark_node, child: *mut cmark_node) -> c_int;
    pub fn cmark_node_append_child(node: *mut cmark_node, child: *mut cmark_node) -> c_int;
    pub fn cmark_consolidate_text_nodes(root: *mut cmark_node);

    // Parsing
    pub fn cmark_parser_new(options: c_int) -> *mut cmark_parser;
    pub fn cmark_parser_free(parser: *mut cmark_parser);
    pub fn cmark_parser_feed(parser: *mut cmark_parser, buffer: *const c_char, len: size_t);
    pub fn cmark_parser_finish(parser: *mut cmark_parser) -> *mut cmark_node;
    pub fn cmark_parse_document(buffer: *const c_char, len: size_t, options: c_int) -> *mut cmark_node;

    // Rendering
    pub fn cmark_render_xml(root: *const cmark_node, options: c_int) -> *mut c_char;
    pub fn cmark_render_html(root: *const cmark_node, options: c_int) -> *mut c_char;
    pub fn cmark_render_man(root: *const cmark_node, options: c_int) -> *mut c_char;
    pub fn cmark_render_commonmark(root: *const cmark_node, options: c_int, width: c_int) -> *mut c_char;

    // Version information
    pub static cmark_version: c_int;
    pub static cmark_version_string: *const c_char;
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::{CString, CStr};
    use std::str;
    use libc::c_int;

    #[test]
    fn basic_api() {
        let source = CString::new("**This** is _CommonMark_").unwrap();
        unsafe {
            let html = CStr::from_ptr(cmark_markdown_to_html(source.as_ptr(), source.as_bytes().len() as c_int, 0));
            let html_buf = html.to_bytes();
            let html_str = str::from_utf8(html_buf).unwrap();
            assert!(html_str == "<p><strong>This</strong> is <em>CommonMark</em></p>\n");
        }
    }
}
