////////////////////////////////////////////////////////////////////////////////
// Copyright (C) 2022 Manatsawin Hanmongkolchai
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
////////////////////////////////////////////////////////////////////////////////

pub use alphamap::AlphaMap;
use std::iter;
use std::string::FromUtf8Error;
pub use trie::{StoreError, Trie, TrieIter, TrieState};

mod alphamap;
mod alphamaploader;
#[cfg(feature = "cffi")]
mod cffi;
mod darray;
mod darrayloader;
mod symbols;
mod tail;
mod tailloader;
#[cfg(all(test, feature = "test_cdatrie", not(feature = "cffi")))]
mod test_cdatrie;
#[cfg(test)]
mod test_utils;
mod trie;

#[cfg(all(feature = "test_cdatrie", feature = "cffi"))]
compile_error!("test_cdatrie cannot be enabled with cffi");

/// AlphaChar is the alphabet character used in words of a target language
/// A string of AlphaChar is null delimitered
pub type AlphaChar = u32;
pub type TrieIndex = i32;
/// TrieChar is internal representation of each AlphaChar
/// A string of TrieChar is null delimitered
pub type TrieChar = u8;
pub type TrieData = i32;

/// cbindgen:ignore
pub const ALPHA_CHAR_TERM: AlphaChar = 0;
pub const TRIE_CHAR_TERM: TrieChar = 0;

/// Convert &[AlphaChar] to String
pub fn alphachars_to_string(ac: &[AlphaChar]) -> Result<String, AlphaCharStringError> {
    // FIXME: Use memchr
    match ac {
        &[] | &[ALPHA_CHAR_TERM] => return Ok(String::new()),
        &[.., ALPHA_CHAR_TERM] => {
            let vec = ac
                .iter()
                .map_while(|v| match *v {
                    ALPHA_CHAR_TERM => None,
                    v => Some(v as u8),
                })
                .collect();
            String::from_utf8(vec).map_err(|err| AlphaCharStringError::Utf8Error(err))
        }
        _ => Err(AlphaCharStringError::NotZeroDelimetered),
    }
}

// TODO: Better error type
#[derive(Debug, Eq, PartialEq)]
pub enum AlphaCharStringError {
    NotZeroDelimetered,
    Utf8Error(FromUtf8Error),
}

#[derive(Debug, Eq, PartialEq)]
pub struct NulError();

pub trait ToAlphaChars {
    fn to_alphachars(&self) -> Result<Vec<AlphaChar>, NulError>;
}

impl<'a> ToAlphaChars for &'a str {
    fn to_alphachars(&self) -> Result<Vec<AlphaChar>, NulError> {
        // FIXME: Use memchr
        let has_inner_null = self
            .bytes()
            .find(|v| *v as AlphaChar == ALPHA_CHAR_TERM)
            .is_some();
        if has_inner_null {
            return Err(NulError());
        }
        Ok(self
            .bytes()
            .map(|v| v as AlphaChar)
            .chain(iter::once(ALPHA_CHAR_TERM))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        alphachars_to_string, AlphaCharStringError, NulError, ToAlphaChars, ALPHA_CHAR_TERM,
    };

    #[test]
    fn test_to_from_alphachars() {
        let start = "hello".to_string();
        let alphachars = start.as_str().to_alphachars().unwrap();
        assert_eq!(alphachars_to_string(alphachars.as_slice()), Ok(start));
    }

    #[test]
    fn test_alphachars_to_string_null() {
        assert_eq!(alphachars_to_string(&[]), Ok("".to_string()));
        assert_eq!(alphachars_to_string(&[ALPHA_CHAR_TERM]), Ok("".to_string()));
        assert_eq!(
            alphachars_to_string(&[1, 2, 3]),
            Err(AlphaCharStringError::NotZeroDelimetered)
        );
    }

    #[test]
    fn tes_to_alphachars_null() {
        assert_eq!("hello\0world".to_alphachars(), Err(NulError()));
    }
}
