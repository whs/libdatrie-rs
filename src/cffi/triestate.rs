use crate::trie::TrieState;
use crate::{AlphaChar, TrieData};
use std::slice::from_raw_parts_mut;

// TrieState * trie_state_clone (const TrieState *s);
#[no_mangle]
extern "C" fn trie_state_clone<'a>(state: &TrieState<'a>) -> *mut TrieState<'a> {
    let cloned = state.clone();
    Box::into_raw(Box::new(cloned))
}

// void        trie_state_copy (TrieState *dst, const TrieState *src);
#[no_mangle]
extern "C" fn trie_state_copy<'a>(dst: &mut TrieState<'a>, src: &TrieState<'a>) {
    dst.clone_from(src);
}

// void      trie_state_free (TrieState *s);
#[no_mangle]
extern "C" fn trie_state_free(state: *mut TrieState) {
    unsafe { drop(Box::from_raw(state)) }
}

// void      trie_state_rewind (TrieState *s);

// Bool      trie_state_walk (TrieState *s, AlphaChar c);
#[no_mangle]
extern "C" fn trie_state_walk(state: &mut TrieState, c: AlphaChar) -> bool {
    state.walk(c)
}

// Bool      trie_state_is_walkable (const TrieState *s, AlphaChar c);
#[no_mangle]
extern "C" fn trie_state_is_walkable(state: &TrieState, c: AlphaChar) -> bool {
    state.is_walkable(c)
}

#[no_mangle]
extern "C" fn trie_state_walkable_chars(
    state: &TrieState,
    chars: *mut AlphaChar,
    chars_nelm: i32,
) -> i32 {
    // int       trie_state_walkable_chars (const TrieState  *s,
    //                                      AlphaChar         chars[],
    //                                      int               chars_nelm);
    let chars = unsafe { from_raw_parts_mut(chars, chars_nelm as usize) };
    state.walkable_chars(chars)
}

// Bool      trie_state_is_single (const TrieState *s);
#[no_mangle]
extern "C" fn trie_state_is_single(state: &TrieState) -> bool {
    state.is_single()
}

// TrieData trie_state_get_data (const TrieState *s);
#[no_mangle]
extern "C" fn trie_state_get_data(state: &TrieState) -> TrieData {
    state.get_data().unwrap()
}
