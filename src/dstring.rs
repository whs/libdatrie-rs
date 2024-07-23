use crate::types::*;
use libc;
use std::slice;

#[derive(Clone)]
#[repr(C)]
pub struct DString {
    char_size: usize,
    str_len: usize,
    // Don't trust val.len(), use str_len * char_size for actual contents
    val: Vec<u8>,
}

#[must_use]
#[no_mangle]
pub extern "C" fn dstring_new(char_size: i32, n_elm: i32) -> *mut DString {
    let dstring = DString {
        char_size: char_size as usize,
        str_len: 0,
        val: vec![0; (char_size * n_elm) as usize],
    };
    Box::into_raw(Box::new(dstring))
}

#[no_mangle]
pub unsafe extern "C" fn dstring_free(ds: *mut DString) {
    let dstring = Box::from_raw(ds);
    drop(dstring) // This is not strictly needed, but it help in clarity
}

#[no_mangle]
pub extern "C" fn dstring_length(ds: *const DString) -> i32 {
    let ds = unsafe { &*ds };
    ds.str_len as i32
}

#[no_mangle]
pub extern "C" fn dstring_get_val(ds: *const DString) -> *const libc::c_void {
    unsafe { (*ds).val.as_ptr().cast() }
}

#[no_mangle]
pub extern "C" fn dstring_get_val_rw(ds: *mut DString) -> *mut libc::c_void {
    unsafe { (*ds).val.as_mut_ptr().cast() }
}

#[no_mangle]
pub extern "C" fn dstring_clear(ds: *mut DString) {
    unsafe {
        (*ds).str_len = 0;
    }
}

#[no_mangle]
#[must_use]
pub extern "C" fn dstring_copy(dst: *mut DString, src: *const DString) -> Bool {
    let dst = unsafe { &mut *dst };
    let src = unsafe { &*src };

    // Unlike clone() this should not reallocate if not necessary
    dst.val.resize(src.val.len(), 0);
    dst.val.copy_from_slice(&src.val);

    dst.char_size = src.char_size;
    dst.str_len = src.str_len;

    TRUE
}

#[no_mangle]
pub extern "C" fn dstring_append(dst: *mut DString, src: *const DString) -> Bool {
    let dst = unsafe { &mut *dst };
    let src = unsafe { &*src };

    if dst.char_size != src.char_size {
        return FALSE;
    }

    dst.val
        .resize((dst.str_len + src.str_len + 1) * dst.char_size, 0);

    for (dchr, schr) in dst
        .val
        .iter_mut()
        .skip(dst.char_size * dst.str_len)
        .take((src.str_len + 1) * dst.char_size)
        .zip(&src.val)
    {
        *dchr = *schr;
    }
    dst.str_len += src.str_len;
    TRUE
}

#[no_mangle]
pub extern "C" fn dstring_append_string(
    ds: *mut DString,
    data: *const libc::c_void,
    len: i32,
) -> Bool {
    let ds = unsafe { &mut *ds };
    let data = unsafe { slice::from_raw_parts(data.cast(), ds.char_size * len as usize) };

    ds.val
        .resize((ds.str_len + len as usize + 1) * ds.char_size, 0);

    for (dchr, schr) in ds
        .val
        .iter_mut()
        .skip(ds.char_size * ds.str_len)
        .take(ds.char_size * len as usize)
        .zip(data)
    {
        *dchr = *schr;
    }
    ds.str_len += len as usize;
    TRUE
}

#[no_mangle]
pub extern "C" fn dstring_append_char(ds: *mut DString, data: *const libc::c_void) -> Bool {
    let ds = unsafe { &mut *ds };
    let data = unsafe { slice::from_raw_parts(data.cast(), ds.char_size) };

    ds.val.resize((ds.str_len + 2) * ds.char_size, 0);

    for (dst, src) in ds
        .val
        .iter_mut()
        .skip(ds.char_size * ds.str_len)
        .take(ds.char_size)
        .zip(data)
    {
        *dst = *src;
    }
    ds.str_len += 1;

    TRUE
}
#[no_mangle]
pub extern "C" fn dstring_terminate(ds: *mut DString) -> Bool {
    let ds = unsafe { &mut *ds };
    ds.val.resize((ds.str_len + 2) * ds.char_size, 0);

    for dchar in ds
        .val
        .iter_mut()
        .skip(ds.char_size * ds.str_len)
        .take(ds.char_size)
    {
        *dchar = 0;
    }

    TRUE
}

#[no_mangle]
pub extern "C" fn dstring_cut_last(ds: *mut DString) -> Bool {
    let ds = unsafe { &mut *ds };

    if ds.str_len == 0 {
        return FALSE;
    }

    ds.str_len -= 1;
    TRUE
}
