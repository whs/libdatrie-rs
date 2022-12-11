use crate::alphamap::ALPHAMAP_SIGNATURE;
use crate::AlphaChar;
use byteorder::{BigEndian, ReadBytesExt};
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;

pub struct AlphaMapLoader<'a, T> {
    reader: &'a mut T,
    ranges_left: i32,
}

impl<'a, T: Read> AlphaMapLoader<'a, T> {
    pub fn new(reader: &'a mut T) -> io::Result<Self> {
        if reader.read_u32::<BigEndian>()? != ALPHAMAP_SIGNATURE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid alphamap magic number",
            ));
        }

        let total_ranges = reader.read_i32::<BigEndian>()?;

        Ok(Self {
            reader,
            ranges_left: total_ranges,
        })
    }

    fn next_item(&mut self) -> io::Result<(AlphaChar, AlphaChar)> {
        let b = self.reader.read_i32::<BigEndian>()? as u32;
        let e = self.reader.read_i32::<BigEndian>()? as u32;
        self.ranges_left -= 1;

        Ok((b, e))
    }
}

impl<'a, T: Seek> AlphaMapLoader<'a, T> {
    pub fn skip_all(&mut self) -> io::Result<()> {
        self.reader
            .seek(SeekFrom::Current(
                size_of::<u32>() as i64 * 2 * self.ranges_left as i64,
            ))
            .and(Ok(()))
    }
}

impl<'a, T: Read> Iterator for AlphaMapLoader<'a, T> {
    type Item = io::Result<(AlphaChar, AlphaChar)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ranges_left == 0 {
            return None;
        }

        Some(self.next_item())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.ranges_left as usize, Some(self.ranges_left as usize))
    }
}

impl<'a, T: Read> ExactSizeIterator for AlphaMapLoader<'a, T> {}
