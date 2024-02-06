use crate::{AlphaChar, AlphaMap};

/// Create a new empty alphabet map. The map contents can then be added with
///  [alpha_map_add_range]
///
///  The created object must be freed with [alpha_map_free]
#[no_mangle]
extern "C" fn alpha_map_new() -> *mut AlphaMap {
    // AlphaMap * alpha_map_new (void)
    let map = AlphaMap::new();

    Box::into_raw(Box::new(map))
}

/// Create a clone of alphabet map
///
/// The created object must be freed with [alpha_map_free]
#[no_mangle]
extern "C" fn alpha_map_clone(map: &AlphaMap) -> *mut AlphaMap {
    // AlphaMap *  alpha_map_clone (const AlphaMap *a_map);
    let map = map.clone();

    Box::into_raw(Box::new(map))
}

/// Free an alphabet map object
#[no_mangle]
extern "C" fn alpha_map_free(map: *mut AlphaMap) {
    // void        alpha_map_free (AlphaMap *alpha_map);
    unsafe {
        drop(Box::from_raw(map));
    }
}

/// Add a range of character codes from begin to end to the
/// alphabet set.
#[no_mangle]
extern "C" fn alpha_map_add_range(map: &mut AlphaMap, begin: AlphaChar, end: AlphaChar) -> i32 {
    // int         alpha_map_add_range (AlphaMap  *alpha_map,
    //                                  AlphaChar  begin,
    //                                  AlphaChar  end);
    map.add_range(begin, end);
    0
}

#[cfg(test)]
mod tests {
    use crate::cffi::alphamap::*;

    #[test]
    fn test_c_alphamap() {
        let alphamap = alpha_map_new();
        unsafe {
            alpha_map_add_range(&mut *alphamap, 0, 0xFF);
            assert_eq!((&*alphamap).char_to_trie(255), Some(256));
            alpha_map_free(alpha_map_clone(&*alphamap));
            alpha_map_free(alphamap);
        }
    }
}
