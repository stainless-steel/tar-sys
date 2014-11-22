//! Facilitation of static linking with [libtar][1].
//!
//! [1]: http://www.feep.net/libtar/

extern crate libc;

use libc::{c_char, c_int, c_long, c_void};

// https://github.com/stainless-steel/libtar/blob/master/lib/libtar.h

#[repr(C)]
#[allow(dead_code)]
pub struct tar_header {
    name: [c_char, ..100],
    mode: [c_char, ..8],
    uid: [c_char, ..8],
    gid: [c_char, ..8],
    size: [c_char, ..12],
    mtime: [c_char, ..12],
    chksum: [c_char, ..8],
    typeflag: char,
    linkname: [c_char, ..100],
    magic: [c_char, ..6],
    version: [c_char, ..2],
    uname: [c_char, ..32],
    gname: [c_char, ..32],
    devmajor: [c_char, ..8],
    devminor: [c_char, ..8],
    prefix: [c_char, ..155],
    padding: [c_char, ..12],
    gnu_longname: *mut char,
    gnu_longlink: *mut char,
}

#[repr(C)]
#[allow(dead_code)]
pub struct TAR {
    typo: *mut c_void,
    pathname: *mut c_char,
    fd: c_long,
    oflags: c_int,
    options: c_int,
    th_buf: tar_header,
    h: *mut c_void,
}

extern {
    // https://github.com/stainless-steel/libtar/blob/master/lib/libtar.h

    pub fn tar_open(t: *mut *mut TAR, pathname: *const c_char, typo: *mut c_void,
                    oflags: c_int, mode: c_int, options: c_int) -> c_int;

    pub fn tar_close(t: *mut TAR) -> c_int;

    pub fn tar_extract_all(t: *mut TAR, prefix: *const c_char) -> c_int;
}
