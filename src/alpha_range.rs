use crate::alpha_map::AlphaChar;
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct AlphaRange {
    pub next: *mut AlphaRange,
    pub begin: AlphaChar,
    pub end: AlphaChar,
}

impl AlphaRange {
    pub fn iter(&self) -> AlphaRangeIter {
        AlphaRangeIter {
            range: self,
            phantom: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> AlphaRangeIterMut {
        AlphaRangeIterMut {
            range: self,
            phantom: PhantomData,
        }
    }
}

pub struct AlphaRangeIter<'a> {
    range: *const AlphaRange,
    phantom: PhantomData<&'a AlphaRange>,
}

impl<'a> Iterator for AlphaRangeIter<'a> {
    type Item = &'a AlphaRange;

    fn next(&mut self) -> Option<Self::Item> {
        let out = unsafe { self.range.as_ref() };
        if let Some(v) = out {
            self.range = v.next
        }
        out
    }
}

pub struct AlphaRangeIterMut<'a> {
    range: *mut AlphaRange,
    phantom: PhantomData<&'a AlphaRange>,
}

impl<'a> Iterator for AlphaRangeIterMut<'a> {
    type Item = &'a mut AlphaRange;

    fn next(&mut self) -> Option<Self::Item> {
        let mut out = unsafe { self.range.as_mut() };
        if let Some(ref mut v) = out {
            self.range = v.next
        }
        out
    }
}
