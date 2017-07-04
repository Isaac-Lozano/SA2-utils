use std::io::{self, Bytes, Read};

pub struct Decoder<R> {
    bytes: Bytes<R>,
    control_bits: u8,
    control_idx: u32,
}

impl<R> Decoder<R>
    where R: Read
{
    pub fn new(read: R) -> Decoder<R> {
        Decoder {
            bytes: read.bytes(),
            control_bits: 0,
            control_idx: 8,
        }
    }

    pub fn decode_to_vec(&mut self) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::new();

        loop {
            if self.read_control_bit()? {
                buffer.push(self.read_u8()?);
            }
            else {
                let sequence_len;
                let offset;

                if self.read_control_bit()? {
                    let lower_byte = self.read_u8()? as u16;
                    let higher_byte = self.read_u8()? as u16;
                    let control_data = higher_byte << 8 | lower_byte;

                    if control_data == 0 {
                        return Ok(buffer);
                    }

                    let raw_sequence_len = control_data & 0b0111;
                    let raw_offset = control_data >> 3;

                    if raw_sequence_len == 0 {
                        sequence_len = self.read_u8()? as usize + 1;
                    }
                    else {
                        sequence_len = raw_sequence_len as usize + 2;
                    }

                    offset = (raw_offset as i32 | 0xFFFFE000u32 as i32) as isize;
                }
                else {
                    let upper_bit = self.read_control_bit()? as u32;
                    let lower_bit = self.read_control_bit()? as u32;
                    let raw_sequence_len = upper_bit << 1 | lower_bit;
                    let raw_offset = self.read_u8()?;

                    sequence_len = raw_sequence_len as usize + 2;
                    offset = (raw_offset as i32 | 0xFFFFFF00u32 as i32) as isize;
                }

                let range_start = (buffer.len() as isize + offset) as usize;
                let range_end = range_start + sequence_len;

                for idx in range_start..range_end {
                    let byte = *buffer.get(idx).unwrap();
                    buffer.push(byte);
                }
            }
        }
    }

    fn read_control_bit(&mut self) -> io::Result<bool> {
        if self.control_idx == 8 {
            self.control_idx = 0;
            self.control_bits = self.bytes.next().unwrap()?;
        }
        self.control_idx += 1;
        let bit = self.control_bits & 1;
        self.control_bits >>= 1;
        Ok(bit == 1)
    }

    fn read_u8(&mut self) -> io::Result<u8> {
        self.bytes.next().unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
