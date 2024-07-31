use std::ptr::NonNull;

use ::libc;
use cstream::BorrowedCStream;

pub(crate) fn wrap_cfile_nonnull(
    file: NonNull<libc::FILE>,
) -> cstream::Io<BorrowedCStream<'static>> {
    unsafe { cstream::Io(BorrowedCStream::borrow_raw(file)) }
}
