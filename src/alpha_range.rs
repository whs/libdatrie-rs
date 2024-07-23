use std::marker::PhantomData;
use crate::alpha_map::AlphaChar;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AlphaRange {
    pub next: *mut AlphaRange,
    pub begin: AlphaChar,
    pub end: AlphaChar,
}

impl AlphaRange {
    pub fn iter(&self) -> impl Iterator<Item=&AlphaRange> {
        AlphaRangeIter{
            range: self,
            phantom: PhantomData,
        }
    }
}

struct AlphaRangeIter<'a> {
    range: *const AlphaRange,
    phantom: PhantomData<&'a AlphaRange>,
}

impl<'a> Iterator for AlphaRangeIter<'a> {
    type Item = &'a AlphaRange;

    fn next(&mut self) -> Option<Self::Item> {
        if self.range.is_null() {
            return None;
        }
        let out = unsafe { &*self.range };
        self.range = out.next;
        Some(out)
    }
}