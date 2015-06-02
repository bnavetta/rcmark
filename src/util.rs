use std::ffi::{CString, CStr};
use std::str;
use libc;

#[doc(hidden)]
pub trait Binding: Sized {
    type Raw;

    unsafe fn from_raw(raw: Self::Raw) -> Self;

    fn raw(&self) -> Self::Raw;
}


impl Binding for i32 {
    type Raw = libc::c_int;

    #[inline]
    unsafe fn from_raw(raw: libc::c_int) -> i32 {
        raw as i32
    }

    #[inline]
    fn raw(&self) -> libc::c_int {
        *self as libc::c_int
    }
}

impl Binding for bool {
    type Raw = libc::c_int;

    #[inline]
    unsafe fn from_raw(raw: libc::c_int) -> bool {
        match raw {
            0 => false,
            1 => true,
            _ => panic!("Cannot convert {} to bool", raw)
        }
    }

    #[inline]
    fn raw(&self) -> libc::c_int {
        match *self {
            true  => 1,
            false => 1,
        }
    }
}

impl<'a> Binding for &'a str {
    type Raw = *const libc::c_char;

    unsafe fn from_raw(raw: *const libc::c_char) -> &'a str {
        str::from_utf8(CStr::from_ptr(raw).to_bytes()).unwrap()
    }

    fn raw(&self) -> *const libc::c_char {
        CString::new(*self).unwrap().as_ptr()
    }
}
