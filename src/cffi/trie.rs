use crate::alphamap::AlphaMap;
use crate::cffi::alphachar::alpha_char_strlen;
use crate::trie::{Trie, TrieState};
use crate::{AlphaChar, TrieData};
use core::slice;
use std::ffi::{c_char, CStr, OsStr};
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use std::mem::ManuallyDrop;
use std::ops::DerefMut;
use std::os::fd::FromRawFd;
use std::os::unix::ffi::OsStrExt;
use std::ptr::null_mut;

pub const TRIE_DATA_ERROR: TrieData = -1;

/// Create a new trie
///
/// Create a new empty trie object based on the given alpha_map alphabet
/// set. The trie contents can then be added and deleted with [trie_store] and
/// [trie_delete] respectively.
///
/// The created object must be freed with [trie_free].
#[no_mangle]
extern "C" fn trie_new(alpha_map: &AlphaMap) -> *mut Trie {
    // Trie *  trie_new (const AlphaMap *alpha_map);
    let trie = Trie::new(alpha_map.clone());
    Box::into_raw(Box::new(trie))
}

/// Create a new trie by loading from a file
///
/// Create a new trie and initialize its contents by loading from the file at
/// given path.
///
/// The created object must be freed with [trie_free].
#[no_mangle]
extern "C" fn trie_new_from_file(path: *const c_char) -> *mut Trie {
    // Trie *  trie_new_from_file (const char *path);
    let path = unsafe { OsStr::from_bytes(CStr::from_ptr(path).to_bytes()) };
    let trie = match Trie::from_file(path) {
        Ok(v) => v,
        Err(_) => return null_mut(),
    };

    Box::into_raw(Box::new(trie))
}

/// Create a new trie by reading from an open file
///
/// Create a new trie and initialize its contents by reading from the open
/// file. After reading, the file pointer is left at the end of the trie data.
/// This can be useful for reading embedded trie index as part of a file data.
///
/// The created object must be freed with [trie_free].
#[no_mangle]
#[cfg(unix)] // TODO: Support windows
extern "C" fn trie_fread(fd: *mut libc::FILE) -> *mut Trie {
    // Trie *  trie_fread (FILE *file);
    // We don't actually own the file, so force it to not run File destructor
    let mut file = ManuallyDrop::new(unsafe { File::from_raw_fd(libc::fileno(fd)) });
    let mut buf = BufReader::new(file.deref_mut());
    let trie = match Trie::from_reader(&mut buf) {
        Ok(v) => v,
        Err(_) => {
            // The file position is undefined behavior at this point
            return null_mut();
        }
    };

    let position = buf.stream_position().unwrap();
    // The documented functionality says that after this function return the pointer is left at the end of trie data
    // BufReader might have advanced the file past that point
    buf.seek(SeekFrom::Start(position)).unwrap();

    Box::into_raw(Box::new(trie))
}

/// Free a trie object
///
/// Destruct the trie and free its allocated memory.
#[no_mangle]
extern "C" fn trie_free(trie: *mut Trie) {
    // void    trie_free (Trie *trie);
    unsafe {
        drop(Box::from_raw(trie));
    }
}

// size_t  trie_get_serialized_size (Trie *trie);

// void    trie_serialize (Trie *trie, uint8 *ptr);

// int     trie_save (Trie *trie, const char *path);

// int     trie_fwrite (Trie *trie, FILE *file);

/// Check pending changes
///
/// Check if the trie is dirty with some pending changes and needs saving
/// to keep the file synchronized.
#[no_mangle]
extern "C" fn trie_is_dirty(trie: &Trie) -> bool {
    trie.is_dirty()
}

/// Retrieve an entry from trie
///
/// Retrieve an entry for the given key from trie. On return,
/// if key is found and o_data is not NULL, *o_data is set
/// to the data associated to key.
#[no_mangle]
extern "C" fn trie_retrieve(trie: &Trie, key: *const AlphaChar, data: *mut TrieData) -> bool {
    // Bool    trie_retrieve (const Trie      *trie,
    //                        const AlphaChar *key,
    //                        TrieData        *o_data);
    let key_arr = unsafe { slice::from_raw_parts(key, alpha_char_strlen(key) as usize) };
    let out = trie.retrieve(key_arr);
    match out {
        Some(v) => {
            if !data.is_null() {
                unsafe { *data = v }
            }
            true
        }
        None => false,
    }
}

/// Store a value for an entry to trie
///
/// Store a data for the given key in trie. If key does not
/// exist in trie, it will be appended. If it does, its current data will
/// be overwritten.
#[no_mangle]
extern "C" fn trie_store(trie: &mut Trie, key: &AlphaChar, data: TrieData) -> bool {
    // Bool    trie_store (Trie *trie, const AlphaChar *key, TrieData data);
    let key_arr = unsafe { slice::from_raw_parts(key, alpha_char_strlen(key) as usize) };
    trie.store(key_arr, data).is_ok()
}

/// Store a value for an entry to trie only if the key is not present
///
/// Store a data for the given key in trie. If key does not
/// exist in trie, it will be inserted. If it does, the function will
/// return failure and the existing value will not be touched.
#[no_mangle]
extern "C" fn trie_store_if_absent(trie: &mut Trie, key: &AlphaChar, data: TrieData) -> bool {
    // Bool    trie_store_if_absent (Trie *trie, const AlphaChar *key, TrieData data);
    let key_arr = unsafe { slice::from_raw_parts(key, alpha_char_strlen(key) as usize) };
    trie.store_if_absent(key_arr, data).is_ok()
}

// Bool    trie_delete (Trie *trie, const AlphaChar *key);

type TrieEnumFunc = extern "C" fn(*const AlphaChar, TrieData, *mut libc::c_void) -> bool;

/// Enumerate entries in trie
///
/// Enumerate all entries in trie. For each entry, the user-supplied
/// enum_func callback function is called, with the entry key and data.
/// Returning FALSE from such callback will stop enumeration and return FALSE.
#[no_mangle]
extern "C" fn trie_enumerate(
    trie: &Trie,
    enum_func: TrieEnumFunc,
    user_data: *mut libc::c_void,
) -> bool {
    // Bool trie_enumerate (const Trie *trie, TrieEnumFunc enum_func, void *user_data)
    for (ch, data) in trie.iter() {
        let cont = enum_func(ch.as_ptr(), data.unwrap_or(TRIE_DATA_ERROR), user_data);
        if !cont {
            return false;
        }
        drop(ch); // Make sure that ch live until this point
    }

    true
}

/// Get root state of a trie
///
/// Get root state of trie, for stepwise walking.
///
/// The returned state is allocated and must be freed with [trie_state_free]
#[no_mangle]
extern "C" fn trie_root(trie: &Trie) -> *mut TrieState {
    // TrieState * trie_root (const Trie *trie);
    let root = trie.root();
    Box::into_raw(Box::new(root))
}
