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

use crate::tail::{TailData, TAIL_SIGNATURE};
use byteorder::{BigEndian, ReadBytesExt};
use std::io;
use std::io::Read;

pub struct TailLoader<'a, T> {
    pub first_free: i32,

    reader: &'a mut T,
    items_left: i32,
}

impl<'a, T: Read> TailLoader<'a, T> {
    pub fn new(reader: &'a mut T) -> io::Result<Self> {
        if reader.read_u32::<BigEndian>()? != TAIL_SIGNATURE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid tail magic number",
            ));
        }

        let first_free = reader.read_i32::<BigEndian>()?;
        let items_left = reader.read_i32::<BigEndian>()?;
        // XXX: Datrie check if num_tails is over usize/sizeof(TailBlock)

        Ok(Self {
            reader,
            first_free,
            items_left,
        })
    }

    fn next_item(&mut self) -> io::Result<TailData> {
        let next_free = match self.reader.read_i32::<BigEndian>()? {
            -1 => None,
            v => Some(v),
        };
        let data = match self.reader.read_i32::<BigEndian>()? {
            -1 => None,
            v => Some(v),
        };
        let length = self.reader.read_i16::<BigEndian>()?.max(0) as usize;

        let mut suffix = vec![0; length];
        self.reader.read_exact(&mut suffix)?;

        self.items_left -= 1;

        Ok(TailData {
            next_free,
            data,
            suffix,
        })
    }
}

impl<'a, T: Read> Iterator for TailLoader<'a, T> {
    type Item = io::Result<TailData>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.items_left == 0 {
            return None;
        }

        Some(self.next_item())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.items_left as usize, Some(self.items_left as usize))
    }
}

impl<'a, T: Read> ExactSizeIterator for TailLoader<'a, T> {}
