use std::io::{self, Write, Seek, SeekFrom};
use std::iter;

use ::{Sample, AdxSpec, gen_coeffs};
use adx_header::{AdxHeader, AdxEncoding, AdxVersion, AdxVersion3LoopInfo, ADX_HEADER_LEN};
use adx_writer::AdxWriter;

const HIGHPASS_FREQ: u32 = 0x01F4;

#[derive(Clone,Copy,Debug)]
struct Block {
    prev: i16,
    prev_prev: i16,
    scale: u16,
    deltas: [i32; 32],
    size: usize,
}

impl Block {
    fn new() -> Block {
        Block {
            prev: 0,
            prev_prev: 0,
            scale: 0,
            deltas: [0; 32],
            size: 0,
        }
    }

    fn from_prev(other: &Block) -> Block {
        Block {
            prev: other.prev,
            prev_prev: other.prev_prev,
            scale: 0,
            deltas: [0; 32],
            size: 0,
        }
    }

    fn push(&mut self, sample: i16, coeffs: (i32, i32)) {
        let prediction_fixed_point = coeffs.0 * self.prev as i32 +
                                     coeffs.1 * self.prev_prev as i32;
        let prediction = prediction_fixed_point >> 12;
        let delta = sample as i32 - prediction;
        self.deltas[self.size] = delta;
        self.size += 1;

//        if delta != 0 {
//            if self.scale == 0 {
//                self.scale = 1;
//            }
//            while delta / (self.scale as i32) > 0x07 || delta / (self.scale as i32) < -0x08 {
//                self.scale += 1;
//            }
//        }

        self.prev_prev = self.prev;
        self.prev = sample;
    }

    fn is_full(&self) -> bool {
        self.size == 32
    }

    fn to_writer<W>(&self, mut writer: W) -> io::Result<()>
        where W: Write
    {
        let mut min = 0;
        let mut max = 0;
        for d in self.deltas.iter() {
            if *d < min {
                min = *d;
            }
            if *d > max {
                max = *d;
            }
        }

        if min == 0 && max == 0 {
            for _ in 0..18 {
                writer.write_u8(0)?;
            }
            return Ok(());
        }

        let mut scale = if max / 7 > min / -8 {
            max / 7
        }
        else {
            min / -8
        };

        if scale == 0 {
            scale = 1;
        }

        writer.write_u16(scale as u16)?;
        for byte_idx in 0..self.deltas.len() / 2 {
            let upper_nibble = (self.deltas[byte_idx * 2] / scale as i32) as u8;
            let lower_nibble = (self.deltas[byte_idx * 2 + 1] / scale as i32) as u8 & 0x0F;
            let byte = upper_nibble << 4 | lower_nibble;
            writer.write_u8(byte)?;
        }
        Ok(())
    }
}

#[derive(Clone,Debug)]
struct Frame {
    blocks: Vec<Block>,
}

impl Frame {
    fn new(channels: usize) -> Frame {
        Frame {
            blocks: iter::repeat(Block::new()).take(channels).collect(),
        }
    }

    fn from_prev(other: &Frame) -> Frame {
        let mut blocks = Vec::new();

        for block in other.blocks.iter() {
            blocks.push(Block::from_prev(block));
        }

        Frame {
            blocks: blocks,
        }
    }

    fn push(&mut self, sample: Sample, coeffs: (i32, i32)) {
        for (channel, block) in self.blocks.iter_mut().enumerate() {
            block.push(sample[channel], coeffs);
        }
    }

    fn is_full(&self) -> bool {
        self.blocks[0].is_full()
    }

    fn to_writer<W>(&self, mut writer: W) -> io::Result<()>
        where W: Write
    {
        for block in self.blocks.iter() {
            block.to_writer(&mut writer)?;
        }
        Ok(())
    }
}

#[derive(Clone,Debug)]
pub struct StandardEncoder<W> {
    inner: W,
    spec: AdxSpec,
    coeffs: (i32, i32),
    samples_encoded: usize,
    current_frame: Frame,
}

impl<W> StandardEncoder<W>
    where W: Write + Seek
{
    pub fn new(mut writer: W, spec: AdxSpec) -> io::Result<StandardEncoder<W>> {
        writer.seek(SeekFrom::Start(ADX_HEADER_LEN as u64))?;
        Ok(
            StandardEncoder {
                inner: writer,
                spec: spec,
                coeffs: gen_coeffs(HIGHPASS_FREQ, spec.sample_rate),
                samples_encoded: 0,
                current_frame: Frame::new(spec.channels as usize),
            }
        )
    }

    pub fn encode_data<I>(&mut self, samples: I) -> io::Result<()>
        where I: IntoIterator<Item = Sample>
    {
        for sample in samples {
            self.current_frame.push(sample, self.coeffs);
            if self.current_frame.is_full() {
                self.current_frame.to_writer(&mut self.inner)?;
                let new_frame = Frame::from_prev(&self.current_frame);
                self.current_frame = new_frame;
            }
        }
        Ok(())
    }

    pub fn finish(mut self) -> io::Result<()> {
        self.inner.write_u16(0x8001)?;
        self.inner.write_u16(0x000e)?;
        for _ in 0..14 {
            self.inner.write_u8(0x00)?;
        }
        self.inner.seek(SeekFrom::Start(0))?;
        let header = AdxHeader {
            encoding: AdxEncoding::Standard,
            block_size: 18,
            sample_bitdepth: 4,
            channel_count: self.spec.channels as u8,
            sample_rate: self.spec.sample_rate,
            total_samples: self.samples_encoded as u32,
            highpass_frequency: HIGHPASS_FREQ as u16,
            version: AdxVersion::Version3(None),
            flags: 0,
        };
        header.to_writer(self.inner)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Block;
    use ::gen_coeffs;

    #[test]
    fn test_block_write() {
        let coeffs = gen_coeffs(500, 32000);
        let mut buf = Vec::new();
        let mut block = Block::new();
        for _ in 0..32 {
            block.push(100, coeffs);
        }
        block.to_writer(&mut buf).unwrap();
        println!("{:#?}", block);
        block = Block::from_prev(&block);
        for _ in 0..32 {
            block.push(1, coeffs);
        }
        block.to_writer(&mut buf).unwrap();
        println!("{:#?}", block);
        println!("{:?}", buf);
        assert!(false);
    }
}
