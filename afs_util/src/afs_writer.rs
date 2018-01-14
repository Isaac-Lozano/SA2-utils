use std::io::{self, Read, Write, Seek, SeekFrom};
use std::iter::FromIterator;

use byteorder::{LE, WriteBytesExt};

#[derive(Clone,Copy,Debug)]
struct AfsFile {
    offset: u32,
    size: u32,
}

#[derive(Clone,Debug)]
pub struct AfsWriter<S, I> {
    inner: S,
    files: I,
}

impl<S, I> AfsWriter<S, I>
    where S: Write + Seek,
          I: IntoIterator,
          I::Item: Read,
          I::IntoIter: ExactSizeIterator,
{
    pub fn new(inner: S, iterable: I) -> AfsWriter<S, I> {
        AfsWriter {
            inner: inner,
            files: iterable,
        }
    }

    // TODO: We can potentially have bad things happen if offset goes past 4GB
    pub fn write(mut self) -> io::Result<()> {
        let mut file_iter = self.files.into_iter();
        let num_files = file_iter.len();

        // Size of file entry is 8
        // Plus 8 more bytes for header
        let mut offset = (file_iter.len() + 1) as u64 * 8;

        // Seek to data start and start writing files
        self.inner.seek(SeekFrom::Start(offset))?;

        let mut file_headers = Vec::new();
        for mut file in file_iter {
            let len = io::copy(&mut file, &mut self.inner)?;

            file_headers.push(AfsFile {
                offset: offset as u32,
                size: len as u32,
            });

            offset += len;
        }

        // Go back to the start and write header info
        self.inner.seek(SeekFrom::Start(0))?;

        self.inner.write_all(b"AFS\x00")?;
        self.inner.write_u32::<LE>(num_files as u32)?;

        for file_header in file_headers {
            self.inner.write_u32::<LE>(file_header.offset)?;
            self.inner.write_u32::<LE>(file_header.size)?;
        }

        Ok(())
    }
}
