extern crate byteorder;

mod adx_header;
mod adx_reader;
mod decoder;

use std::io::{self, Read, Seek};

use adx_header::AdxHeader;
use decoder::{Decoder, StandardDecoder};

// #[derive(Clone,Debug)]
pub struct AdxDecoder<S> {
    inner: S,
    decoder: Box<Decoder>,
}

impl<S> AdxDecoder<S>
    where S: Read + Seek
{
    // TODO: Remove seek restraint.
    // We can keep track of copyright_offset as we read
    // and decrement it at key points.
    // Then we just read copyright_offset -2 bytes.
    pub fn new(mut inner: S) -> io::Result<AdxDecoder<S>> {
        let header = AdxHeader::read_header(&mut inner)?;
        let decoder = Box::new(StandardDecoder::from_header(header));

        Ok(AdxDecoder {
            inner: inner,
            decoder: decoder,
        })
    }

    pub fn channels(&self) -> u32 {
        self.decoder.channels()
    }

    pub fn sample_rate(&self) -> u32 {
        self.decoder.sample_rate()
    }
}

impl<S> Iterator for AdxDecoder<S>
    where S: Read
{
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        self.decoder.next_sample(&mut self.inner)
    }
}

type Sample = Vec<i16>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
