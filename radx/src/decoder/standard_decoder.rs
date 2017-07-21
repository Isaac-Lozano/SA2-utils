use std::cmp;
use std::f64;
use std::i16;
use std::io::{self, Seek, Read};
use std::iter;

use adx_header::AdxHeader;
use adx_reader::AdxReader;
use decoder::Decoder;
use ::Sample;

pub struct StandardDecoder<S> {
    inner: S,
    header: AdxHeader,
    samples: Vec<Sample>,
    sample_vec_idx: usize,
    prev_sample: Sample,
    prev_prev_sample: Sample,
    coeff1: i32,
    coeff2: i32,
}

impl<S> StandardDecoder<S>
    where S: Read + Seek
{
    pub fn from_header(header: AdxHeader, inner: S) -> StandardDecoder<S> {
        let (coeff1, coeff2) = gen_coeffs(&header);
        let prev_sample = iter::repeat(0).take(header.channel_count as usize).collect();
        let prev_prev_sample = iter::repeat(0).take(header.channel_count as usize).collect();

        StandardDecoder {
            inner: inner,
            header: header,
            samples: Vec::new(),
            sample_vec_idx: 0,
            prev_sample: prev_sample,
            prev_prev_sample: prev_prev_sample,
            coeff1: coeff1,
            coeff2: coeff2,
        }
    }

    fn read_frame(&mut self) -> io::Result<Option<Vec<Sample>>> {
        let mut bitreader = BitReader::new(&mut self.inner);
        let samples_per_block = ((self.header.block_size as u32 - 2) * 8) / self.header.sample_bitdepth as u32;
        let mut samples: Vec<Sample> = iter::repeat(iter::repeat(0).take(self.header.channel_count as usize).collect())
            .take(samples_per_block as usize).collect();

        for channel in 0..self.header.channel_count as usize {
            let raw_scale = bitreader.read(16)?;
            if raw_scale == 0x8001 {
                return Ok(None);
            }

            let scale = raw_scale as i32;

            for sample_idx in 0..samples_per_block as usize {
                // Predict next sample
                let prediction_fixed_point = self.coeff1 * self.prev_sample[channel] as i32 +
                                             self.coeff2 * self.prev_prev_sample[channel] as i32;

                // Convert to integer
                let prediction = prediction_fixed_point >> 12;

                // Get delta
                let delta = scale * sign_extend(bitreader.read(self.header.sample_bitdepth as u32)?, self.header.sample_bitdepth as u32);

                // Calculate sample
                let unclamped_sample = prediction as i32 + delta;

                // Clamp sample
                let sample = if unclamped_sample >= i16::MAX as i32 {
                    i16::MAX
                }
                else if unclamped_sample <= i16::MIN as i32 {
                    i16::MIN
                }
                else {
                    unclamped_sample as i16
                };

//                println!("[{}, {}] pred: {}, delta: {}, result: {}", sample_idx, channel, prediction, delta, sample);

                self.prev_prev_sample[channel] = self.prev_sample[channel];
                self.prev_sample[channel] = sample;
                samples[sample_idx][channel] = sample;
            }
        }
        Ok(Some(samples))
    }
}

impl<S> Decoder for StandardDecoder<S>
    where S: Seek + Read
{
    fn channels(&self) -> u32 {
        self.header.channel_count as u32
    }

    fn sample_rate(&self) -> u32 {
        self.header.sample_rate as u32
    }

    fn next_sample(&mut self) -> Option<Sample>
    {
        if self.sample_vec_idx == self.samples.len() {
            self.samples = match self.read_frame().unwrap_or(None) {
                Some(v) => v,
                None => return None,
            };
            self.sample_vec_idx = 0;
        }
        let result = self.samples[self.sample_vec_idx].clone();
        self.sample_vec_idx += 1;
        Some(result)
    }
}

/// Returns 12-bit fixed-point coefficients.
fn gen_coeffs(header: &AdxHeader) -> (i32, i32) {
    let highpass_samples = header.highpass_frequency as f64 / header.sample_rate as f64;
    let a = f64::consts::SQRT_2 - (2.0 * f64::consts::PI * highpass_samples).cos();
    let b = f64::consts::SQRT_2 - 1.0;
    let c = (a - ((a + b) * (a - b)).sqrt()) / b;

    let coeff1 = c * 2.0;
    let coeff2 = -(c * c);

    // 4096 = 1**12
    ((coeff1 * 4096.0) as i32, (coeff2 * 4096.0) as i32)
}

fn sign_extend(num: u32, bits: u32) -> i32 {
    let bits_to_shift = 32 - bits;
    (num << bits_to_shift) as i32 >> bits_to_shift
}

struct BitReader<R> {
    inner: R,
    buffer: u8,
    bits_left: u32,
}

impl<R> BitReader<R>
    where R: Read
{
    fn new(inner: R) -> BitReader<R> {
        BitReader {
            inner: inner,
            buffer: 0,
            bits_left: 0,
        }
    }

    fn read_from_buffer(&mut self, bits: u32) -> u32 {
        assert!(bits <= 8);

        let result = self.buffer >> (8 - bits);
        self.buffer = self.buffer.checked_shl(bits).unwrap_or(0);
        self.bits_left -= bits;
        result as u32
    }

    fn read(&mut self, mut bits: u32) -> io::Result<u32> {
        assert!(bits <= 32);

        let mut result = 0;

        while bits != 0 {
            if self.bits_left == 0 {
                self.buffer = self.inner.read_u8()?;
                self.bits_left = 8;
            }
            let bits_to_read = cmp::min(bits, self.bits_left);
            let data = self.read_from_buffer(bits_to_read);
            result = (result << bits_to_read) | data;
            bits -= bits_to_read;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::BitReader;

    #[test]
    fn bitreader() {
        let data: Vec<u8> = vec![0xaa, 0xab, 0xa5, 0x80, 0xff, 0xff, 0x00, 0x00];
        let mut br = BitReader::new(data.as_slice());

        assert_eq!(br.read(16).unwrap(), 0xaaab);
        assert_eq!(br.read(4).unwrap(), 0xa);
        assert_eq!(br.read(4).unwrap(), 0x5);
        assert_eq!(br.read(2).unwrap(), 0x2);
        assert_eq!(br.read(8).unwrap(), 0x3);
        assert_eq!(br.read(30).unwrap(), 0x3fff0000);
    }
}
