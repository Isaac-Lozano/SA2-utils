use std::io::{self, Read, Seek, SeekFrom};

use adx_reader::AdxReader;

const ADX_MAGIC: u16 = 0x8000;

#[derive(Clone,Copy,Debug)]
pub enum AdxVersion {
    Version3 {
        loop_alignment_samples: u16,
        loop_enabled_short: u16,
        loop_enabled_int: u32,
        loop_begin_sample: u32,
        loop_begin_byte: u32,
        loop_end_sample: u32,
        loop_end_byte: u32,
    },
    Version4,
    /// Version 4 without looping support
    Version5,
    /// Seen in SA2B voice afs
    Version6,
}

#[derive(Clone,Copy,Debug)]
pub enum AdxEncoding {
    Preset,
    Standard,
    Exponential,
    Ahx,
}

#[derive(Clone,Debug)]
pub struct AdxHeader {
    pub encoding: AdxEncoding,
    pub block_size: u8,
    pub sample_bitdepth: u8,
    pub channel_count: u8,
    pub sample_rate: u32,
    pub total_samples: u32,
    pub highpass_frequency: u16,
    pub version: AdxVersion,
    pub flags: u8,
}

impl AdxHeader {
    pub(crate) fn read_header<S>(inner: &mut S) -> io::Result<AdxHeader>
        where S: Read + Seek
    {
        let magic = inner.read_u16()?;
        if magic != ADX_MAGIC {
            panic!("Bad magic");
        }

        let data_offset = inner.read_u16()?;
        let encoding = match inner.read_u8()? {
            0x02 => AdxEncoding::Preset,
            0x03 => AdxEncoding::Standard,
            0x04 => AdxEncoding::Exponential,
            0x10 | 0x11 => AdxEncoding::Ahx,
            _ => panic!("Bad encoding"),
        };
        let block_size = inner.read_u8()?;
        let sample_bitdepth = inner.read_u8()?;
        let channel_count = inner.read_u8()?;
        let sample_rate = inner.read_u32()?;
        let total_samples = inner.read_u32()?;
        let highpass_frequency = inner.read_u16()?;
        let version_byte = inner.read_u8()?;
        let flags = inner.read_u8()?;
        let version = match version_byte {
            0x03 => {
                let loop_alignment_samples = inner.read_u16()?;
                let loop_enabled_short = inner.read_u16()?;
                let loop_enabled_int = inner.read_u32()?;
                let loop_begin_sample = inner.read_u32()?;
                let loop_begin_byte = inner.read_u32()?;
                let loop_end_sample = inner.read_u32()?;
                let loop_end_byte = inner.read_u32()?;
                AdxVersion::Version3 {
                    loop_alignment_samples: loop_alignment_samples,
                    loop_enabled_short: loop_enabled_short,
                    loop_enabled_int: loop_enabled_int,
                    loop_begin_sample: loop_begin_sample,
                    loop_begin_byte: loop_begin_byte,
                    loop_end_sample: loop_end_sample,
                    loop_end_byte: loop_end_byte,
                }
            }
            0x04 => AdxVersion::Version4,
            0x05 => AdxVersion::Version5,
            0x06 => AdxVersion::Version6,
            _ => panic!("Bad version"),
        };

        inner.seek(SeekFrom::Start(data_offset as u64 - 2))?;

        let mut copyright_buffer = [0u8; 6];
        inner.read_exact(&mut copyright_buffer)?;
        if copyright_buffer != [0x28, 0x63, 0x29, 0x43, 0x52, 0x49] {
            panic!("Copyright magic wrong");
        }

        Ok(AdxHeader {
            encoding: encoding,
            block_size: block_size,
            sample_bitdepth: sample_bitdepth,
            channel_count: channel_count,
            sample_rate: sample_rate,
            total_samples: total_samples,
            highpass_frequency: highpass_frequency,
            version: version,
            flags: flags,
        })
    }
}
