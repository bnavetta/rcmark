use {raw, Node, CmarkOptions};

use std::ffi::CStr;
use std::str;
use libc;

pub fn render_xml(root: &Node, options: CmarkOptions) -> String {
    unsafe {
        let rendered = CStr::from_ptr(raw::cmark_render_xml(root.raw(), options.raw()));
        let res = str::from_utf8(rendered.to_bytes()).unwrap().to_owned();
        libc::free(rendered.as_ptr() as *mut libc::c_void);
        res
    }
}

pub fn render_html(root: &Node, options: CmarkOptions) -> String {
    unsafe {
        let rendered = CStr::from_ptr(raw::cmark_render_html(root.raw(), options.raw()));
        let res = str::from_utf8(rendered.to_bytes()).unwrap().to_owned();
        libc::free(rendered.as_ptr() as *mut libc::c_void);
        res
    }
}

pub fn render_man(root: &Node, options: CmarkOptions) -> String {
    unsafe {
        let rendered = CStr::from_ptr(raw::cmark_render_man(root.raw(), options.raw()));
        let res = str::from_utf8(rendered.to_bytes()).unwrap().to_owned();
        libc::free(rendered.as_ptr() as *mut libc::c_void);
        res
    }
}

pub fn render_commonmark(root: &Node, options: CmarkOptions, width: i32) -> String {
    unsafe {
        let rendered = CStr::from_ptr(raw::cmark_render_commonmark(root.raw(), options.raw(), width as libc::c_int));
        let res = str::from_utf8(rendered.to_bytes()).unwrap().to_owned();
        libc::free(rendered.as_ptr() as *mut libc::c_void);
        res
    }
}
