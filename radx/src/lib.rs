extern crate byteorder;

mod adx_header;
mod adx_reader;
mod decoder;

use std::io::{self, Read, Seek};

use adx_header::AdxHeader;
use decoder::{Decoder, StandardDecoder};

pub fn from_reader<R>(mut reader: R) -> io::Result<Box<Decoder>>
    where R: Seek + Read + 'static
{
    let header = AdxHeader::read_header(&mut reader)?;
    Ok(Box::new(StandardDecoder::from_header(header, reader, true)))
}

type Sample = Vec<i16>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
