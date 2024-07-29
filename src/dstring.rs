use std::io::Write;
use crate::types::*;
use libc;
use std::mem::MaybeUninit;
use std::{io, mem, slice};

#[derive(Clone)]
pub(crate) struct DString {
    char_size: usize,
    str_len: usize,
    // Don't trust val.len(), use str_len * char_size for actual contents
    val: Vec<MaybeUninit<u8>>,
}

impl Write for DString {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.len() % self.char_size != 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "write must be multiples of char_size"))
        }

        self.val.resize(
            (self.str_len + buf.len()) * self.char_size,
            MaybeUninit::uninit(),
        );

        let start_pos = self.char_size * self.str_len;

        let dst_ptr = &mut self.val[start_pos..(start_pos+buf.len())];
        dst_ptr.copy_from_slice(unsafe { mem::transmute(buf) });

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[must_use]
#[no_mangle]
pub(crate) extern "C" fn dstring_new(char_size: i32, n_elm: i32) -> *mut DString {
    let dstring = DString {
        char_size: char_size as usize,
        str_len: 0,
        val: vec![MaybeUninit::uninit(); (char_size * n_elm) as usize],
    };
    Box::into_raw(Box::new(dstring))
}

#[no_mangle]
pub(crate) unsafe extern "C" fn dstring_free(ds: *mut DString) {
    let dstring = Box::from_raw(ds);
    drop(dstring) // This is not strictly needed, but it help in clarity
}

#[no_mangle]
pub(crate) extern "C" fn dstring_length(ds: *const DString) -> i32 {
    let ds = unsafe { &*ds };
    ds.str_len as i32
}

#[no_mangle]
pub(crate) extern "C" fn dstring_get_val(ds: *const DString) -> *const libc::c_void {
    unsafe { (*ds).val.as_ptr().cast() }
}

#[no_mangle]
pub(crate) extern "C" fn dstring_get_val_rw(ds: *mut DString) -> *mut libc::c_void {
    unsafe { (*ds).val.as_mut_ptr().cast() }
}

#[no_mangle]
pub(crate) extern "C" fn dstring_clear(ds: *mut DString) {
    unsafe {
        (*ds).str_len = 0;
        (*ds).val.clear();
    }
}

#[no_mangle]
#[must_use]
pub(crate) extern "C" fn dstring_copy(dst: *mut DString, src: *const DString) -> Bool {
    let dst = unsafe { &mut *dst };
    let src = unsafe { &*src };
    src.clone_into(dst);
    TRUE
}

#[no_mangle]
pub(crate) extern "C" fn dstring_append(dst: *mut DString, src: *const DString) -> Bool {
    let dst = unsafe { &mut *dst };
    let src = unsafe { &*src };

    if dst.char_size != src.char_size {
        return FALSE;
    }

    dst.val.resize(
        (dst.str_len + src.str_len + 1) * dst.char_size,
        MaybeUninit::uninit(),
    );

    let start_pos = dst.char_size * dst.str_len;
    let copy_len = (src.str_len+1) * dst.char_size;

    let dst_ptr = &mut dst.val[start_pos..(start_pos+copy_len)];
    dst_ptr.copy_from_slice(&src.val[..copy_len]);

    dst.str_len += src.str_len;
    TRUE
}

#[no_mangle]
pub(crate) extern "C" fn dstring_append_string(
    ds: *mut DString,
    data: *const libc::c_void,
    len: i32,
) -> Bool {
    let ds = unsafe { &mut *ds };
    let copy_len = ds.char_size * len as usize;
    let data = unsafe { slice::from_raw_parts(data.cast(), copy_len) };

    ds.val.resize(
        (ds.str_len + len as usize + 1) * ds.char_size,
        MaybeUninit::uninit(),
    );

    let start_pos = ds.char_size * ds.str_len;

    let dst_ptr = &mut ds.val[start_pos..(start_pos+copy_len)];
    debug_assert_eq!(dst_ptr.len(), copy_len);
    dst_ptr.copy_from_slice(data);

    ds.str_len += len as usize;
    TRUE
}

#[no_mangle]
pub(crate) extern "C" fn dstring_append_char(ds: *mut DString, data: *const libc::c_void) -> Bool {
    let ds = unsafe { &mut *ds };
    let data = unsafe { slice::from_raw_parts(data.cast(), ds.char_size) };

    ds.val
        .resize((ds.str_len + 2) * ds.char_size, MaybeUninit::uninit());

    let start_pos = ds.char_size * ds.str_len;
    let copy_len = ds.char_size;

    let dst_ptr = &mut ds.val[start_pos..(start_pos+copy_len)];
    debug_assert_eq!(dst_ptr.len(), copy_len);
    dst_ptr.copy_from_slice(data);

    ds.str_len += 1;

    TRUE
}
#[no_mangle]
pub(crate) extern "C" fn dstring_terminate(ds: *mut DString) -> Bool {
    let ds = unsafe { &mut *ds };
    ds.val
        .resize((ds.str_len + 2) * ds.char_size, MaybeUninit::uninit());

    for dchar in ds
        .val
        .iter_mut()
        .skip(ds.char_size * ds.str_len)
        .take(ds.char_size)
    {
        *dchar = MaybeUninit::new(0);
    }

    TRUE
}

#[no_mangle]
pub(crate) extern "C" fn dstring_cut_last(ds: *mut DString) -> Bool {
    let ds = unsafe { &mut *ds };

    if ds.str_len == 0 {
        return FALSE;
    }

    ds.str_len -= 1;
    TRUE
}
