use crate::types::*;
use ::libc;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use cstream::BorrowedCStream;
use std::io::{Read, Write};
use std::ptr::NonNull;
use std::slice;

pub(crate) fn wrap_cfile(file: *mut libc::FILE) -> Option<cstream::Io<BorrowedCStream<'static>>> {
    NonNull::new(file).map(|file| unsafe { cstream::Io(BorrowedCStream::borrow_raw(file)) })
}

pub(crate) fn wrap_cfile_nonnull(
    file: NonNull<libc::FILE>,
) -> cstream::Io<BorrowedCStream<'static>> {
    unsafe { cstream::Io(BorrowedCStream::borrow_raw(file)) }
}

#[no_mangle]
pub(crate) extern "C" fn file_read_int32(file: *mut libc::FILE, o_val: *mut i32) -> Bool {
    let mut stream = match wrap_cfile(file) {
        Some(v) => v,
        None => return FALSE,
    };
    match stream.read_i32::<BigEndian>() {
        Ok(v) => unsafe {
            *o_val = v;
            TRUE
        },
        Err(_) => FALSE,
    }
}

fn serialize_int32_be(buff: &mut [u8], val: i32) {
    BigEndian::write_i32(buff, val)
}

#[no_mangle]
pub(crate) unsafe extern "C" fn serialize_int32_be_incr(buff: *mut *mut u8, val: i32) {
    let write_buf: *mut [u8] = *buff.cast();
    BigEndian::write_i32(&mut *write_buf, val);
    *buff = (*buff).offset(4);
}

#[no_mangle]
pub(crate) extern "C" fn file_write_int32(file: *mut libc::FILE, val: i32) -> Bool {
    let mut stream = match wrap_cfile(file) {
        Some(v) => v,
        None => return FALSE,
    };
    match stream.write_i32::<BigEndian>(val) {
        Ok(_) => TRUE,
        Err(_) => FALSE,
    }
}

extern "C" fn parse_int16_be(buff: &[u8]) -> i16 {
    BigEndian::read_i16(buff)
}

#[no_mangle]
pub(crate) extern "C" fn file_read_int16(file: *mut libc::FILE, o_val: *mut i16) -> Bool {
    let mut stream = match wrap_cfile(file) {
        Some(v) => v,
        None => return FALSE,
    };
    match stream.read_i16::<BigEndian>() {
        Ok(v) => unsafe {
            *o_val = v;
            TRUE
        },
        Err(_) => FALSE,
    }
}

fn serialize_int16_be(buff: &mut [u8], val: i16) {
    BigEndian::write_i16(buff, val)
}

#[no_mangle]
pub(crate) unsafe extern "C" fn serialize_int16_be_incr(buff: *mut *mut u8, val: i16) {
    let write_buf: *mut [u8] = *buff.cast();
    BigEndian::write_i16(&mut *write_buf, val);
    *buff = (*buff).offset(2);
}

#[no_mangle]
pub(crate) extern "C" fn file_write_int16(file: *mut libc::FILE, val: i16) -> Bool {
    let mut stream = match wrap_cfile(file) {
        Some(v) => v,
        None => return FALSE,
    };
    match stream.write_i16::<BigEndian>(val) {
        Ok(_) => TRUE,
        Err(_) => FALSE,
    }
}

#[no_mangle]
pub(crate) extern "C" fn file_read_int8(file: *mut libc::FILE, o_val: *mut i8) -> Bool {
    let mut stream = match wrap_cfile(file) {
        Some(v) => v,
        None => return FALSE,
    };
    match stream.read_i8() {
        Ok(v) => unsafe {
            *o_val = v;
            TRUE
        },
        Err(_) => FALSE,
    }
}

#[no_mangle]
pub(crate) extern "C" fn file_write_int8(file: *mut libc::FILE, val: i8) -> Bool {
    let mut stream = match wrap_cfile(file) {
        Some(v) => v,
        None => return FALSE,
    };
    match stream.write_i8(val) {
        Ok(_) => TRUE,
        Err(_) => FALSE,
    }
}

#[no_mangle]
pub(crate) extern "C" fn file_read_chars(file: *mut libc::FILE, buff: *mut u8, len: i32) -> Bool {
    let mut stream = match wrap_cfile(file) {
        Some(v) => v,
        None => return FALSE,
    };
    let buff = unsafe { slice::from_raw_parts_mut(buff, len as usize) };

    match stream.read_exact(buff) {
        Ok(_) => TRUE,
        Err(_) => FALSE,
    }
}

#[no_mangle]
pub(crate) extern "C" fn file_write_chars(
    file: *mut libc::FILE,
    buff: *const u8,
    len: i32,
) -> Bool {
    let mut stream = match wrap_cfile(file) {
        Some(v) => v,
        None => return FALSE,
    };
    let buff = unsafe { slice::from_raw_parts(buff, len as usize) };

    match stream.write(buff) {
        Ok(v) if v == len as usize => TRUE,
        Ok(_) => FALSE,
        Err(_) => FALSE,
    }
}
