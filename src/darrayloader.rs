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

use crate::darray::{Cell, DA_SIGNATURE};
use crate::TrieIndex;
use byteorder::{BigEndian, ReadBytesExt};
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;

pub struct DarrayLoader<'a, T> {
    reader: &'a mut T,
    cells_left: i32,
    emit_initial: bool,
}

impl<'a, T: Read> DarrayLoader<'a, T> {
    pub fn new(reader: &'a mut T) -> io::Result<Self> {
        if reader.read_u32::<BigEndian>()? != DA_SIGNATURE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid datrie magic number",
            ));
        }

        let num_cells = reader.read_i32::<BigEndian>()?;
        // XXX: Datrie check if num_cells is over usize/sizeof(DaCell)

        Ok(Self {
            reader,
            cells_left: num_cells,
            emit_initial: false,
        })
    }

    fn next_item(&mut self) -> io::Result<Cell> {
        let base = self.reader.read_i32::<BigEndian>()? as TrieIndex;
        let check = self.reader.read_i32::<BigEndian>()? as TrieIndex;

        self.cells_left -= 1;

        Ok(Cell { base, check })
    }
}

impl<'a, T: Seek> DarrayLoader<'a, T> {
    pub fn skip_all(&mut self) -> io::Result<()> {
        let mut left = self.cells_left as i64;
        if !self.emit_initial {
            // initial cell is not written on disk
            left -= 1;
        }
        self.reader
            .seek(SeekFrom::Current(size_of::<TrieIndex>() as i64 * 2 * left))
            .and(Ok(()))
    }
}

impl<'a, T: Read> Iterator for DarrayLoader<'a, T> {
    type Item = io::Result<Cell>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.emit_initial {
            let cells_left = self.cells_left;
            self.cells_left -= 1;
            self.emit_initial = true;

            return Some(Ok(Cell {
                base: DA_SIGNATURE as TrieIndex,
                check: cells_left,
            }));
        }

        if self.cells_left == 0 {
            return None;
        }

        Some(self.next_item())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.cells_left as usize, Some(self.cells_left as usize))
    }
}

impl<'a, T: Read> ExactSizeIterator for DarrayLoader<'a, T> {}
