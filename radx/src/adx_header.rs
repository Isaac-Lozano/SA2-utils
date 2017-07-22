use std::io::{self, Read, Seek, SeekFrom};

use adx_reader::AdxReader;

const ADX_MAGIC: u16 = 0x8000;

#[derive(Clone,Copy,Debug)]
pub struct AdxVersion3LoopInfo {
    pub alignment_samples: u16,
    pub enabled_short: u16,
    pub enabled_int: u32,
    pub begin_sample: u32,
    pub begin_byte: u32,
    pub end_sample: u32,
    pub end_byte: u32,
}

#[derive(Clone,Debug)]
pub enum AdxVersion {
    Version3(Option<AdxVersion3LoopInfo>),
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
    pub data_offset: u16,
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
                let loop_info = if data_offset >= 40 { 
                    let alignment_samples = inner.read_u16()?;
                    let enabled_short = inner.read_u16()?;
                    let enabled_int = inner.read_u32()?;
                    let begin_sample = inner.read_u32()?;
                    let begin_byte = inner.read_u32()?;
                    let end_sample = inner.read_u32()?;
                    let end_byte = inner.read_u32()?;
                    Some(
                        AdxVersion3LoopInfo {
                            alignment_samples: alignment_samples,
                            enabled_short: enabled_short,
                            enabled_int: enabled_int,
                            begin_sample: begin_sample,
                            begin_byte: begin_byte,
                            end_sample: end_sample,
                            end_byte: end_byte,
                        }
                    )
                }
                else {
                    None
                };
                AdxVersion::Version3(loop_info)
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
            data_offset: data_offset,
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
