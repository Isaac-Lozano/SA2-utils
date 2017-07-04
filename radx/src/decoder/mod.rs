mod standard_decoder;

use std::io::Read;

pub(crate) use self::standard_decoder::StandardDecoder;

use ::Sample;

pub trait Decoder {
    fn channels(&self) -> u32;
    fn sample_rate(&self) -> u32;
    fn next_sample(&mut self, inner: &mut Read) -> Option<Sample>;
}
