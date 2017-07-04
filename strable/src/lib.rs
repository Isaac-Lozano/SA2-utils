extern crate byteorder;

use std::io::{self, Read, Write, Seek, SeekFrom};

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

#[derive(Clone,Debug)]
pub struct Strable {
    data_table: Vec<Vec<u8>>,
}

impl Strable {
    pub fn from_seek<S>(seekable: &mut S) -> io::Result<Strable>
        where S: Read + Seek
    {
        let mut offsets = Vec::new();

        loop {
            let offset = seekable.read_u32::<BigEndian>()?;
            if offset == 0xffffffff {
                break;
            }
            offsets.push(offset);
        }

        let mut data_table = Vec::with_capacity(offsets.len());

        for offset in offsets {
            seekable.seek(SeekFrom::Start(offset as u64))?;

            let data = seekable.bytes()
                .map(|b| b.unwrap_or(0))
                .take_while(|b| *b != 0)
                .collect();

            data_table.push(data);
        }

        Ok(Strable {
            data_table: data_table,
        })
    }

    pub fn write_data<W>(&self, writeable: &mut W) -> io::Result<()>
        where W: Write
    {
        // table size = (#entries + 1) * sizeof(u32);
        // extra 1 is for terminator entry
        let mut data_ptr = (self.data_table.len() + 1) * 4;

        for data in self.data_table.iter() {
            writeable.write_u32::<BigEndian>(data_ptr as u32)?;
            // extra 1 is for null terminator
            data_ptr += data.len() + 1;
        }

        // table terminator
        writeable.write_u32::<BigEndian>(0xffffffff)?;

        for data in self.data_table.iter() {
            writeable.write_all(data)?;
            writeable.write_u8(0)?;
        }

        Ok(())
    }

    pub fn into_vec(self) -> Vec<Vec<u8>> {
        self.data_table
    }

    pub fn from_vec(data_table: Vec<Vec<u8>>) -> Strable {
        Strable {
            data_table: data_table,
        }
    }

    pub fn strings<'a>(&'a mut self) -> StrableIterator<'a> {
        StrableIterator {
            data_iter: self.data_table.iter(),
        }
    }
}

#[derive(Debug)]
pub struct StrableIterator<'a> {
    data_iter: std::slice::Iter<'a, Vec<u8>>,
}

impl<'a> Iterator for StrableIterator<'a> {
    type Item = &'a Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.data_iter.next()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
