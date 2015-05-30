extern crate libc;

use libc::{c_int, c_char};

extern {
    pub fn cmark_markdown_to_html(text: *const c_char, len: c_int, options: c_int) -> *mut c_char;
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
