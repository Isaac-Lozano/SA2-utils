extern crate byteorder;

use std::io::{self, Read, Write, Seek, SeekFrom};

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

#[derive(Clone,Debug)]
pub struct Strable {
    pub data_table: Vec<Vec<u8>>,
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

    pub fn to_writer<W>(&self, mut writer: W) -> io::Result<()>
        where W: Write
    {
        // table size = (#entries + 1) * sizeof(u32);
        // extra 1 is for terminator entry
        let mut data_ptr = (self.data_table.len() + 1) * 4;

        for data in self.data_table.iter() {
            writer.write_u32::<BigEndian>(data_ptr as u32)?;
            // extra 1 is for null terminator
            data_ptr += data.len() + 1;
        }

        // table terminator
        writer.write_u32::<BigEndian>(0xffffffff)?;

        for data in self.data_table.iter() {
            writer.write_all(data)?;
            writer.write_u8(0)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
