#[derive(Clone,Copy,Debug)]
struct Match {
    offset: isize,
    length: usize,
}

#[derive(Clone,Debug)]
pub struct Encoder<'a> {
    data: &'a [u8],
    buffer: Vec<u8>,
    control_byte_idx: usize,
    control_byte_bit: u32,
}

impl<'a> Encoder<'a> {
    pub fn new(data: &[u8]) -> Encoder {
        Encoder {
            data: data,
            buffer: Vec::new(),
            control_byte_idx: 0,
            control_byte_bit: 8,
        }
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut encode_idx = 0;

        while encode_idx < self.data.len() {
            let bytes_encoded;
            let best_match_opt = self.get_best_match(encode_idx);
            if let Some(best_match) = best_match_opt {
                if best_match.length <= 5 && best_match.offset > -0x100 {
                    self.encode_shortcopy(best_match);
                }
                else {
                    self.encode_longcopy(best_match);
                }
                bytes_encoded = best_match.length;
            }
            else {
                let byte = self.data[encode_idx];
                self.encode_literal(byte);
                bytes_encoded = 1;
            }
            encode_idx += bytes_encoded;
        }

        self.encode_finish();
        self.buffer
    }

    fn get_best_match(&self, idx: usize) -> Option<Match> {
        let mut best_match: Option<Match> = None;
        let mut search_idx = if idx < 0x1FFF {
            0
        }
        else {
            idx - 0x1FFF
        };

        while search_idx < idx {
            let mut match_len = 0;
            while idx + match_len < self.data.len() &&
                match_len < 0x101 && 
                self.data[search_idx + match_len] == self.data[idx + match_len]
            {
                match_len += 1;
            }

            if match_len >= 3 &&
                (best_match.is_none() || best_match.unwrap().length < match_len)
            {
                best_match = Some(
                        Match {
                        offset: search_idx as isize - idx as isize,
                        length: match_len,
                    }
                )
            }

            search_idx += 1;
        }

        best_match
    }

    fn encode_literal(&mut self, literal: u8) {
        self.encode_control(1);
        self.buffer.push(literal);
    }

    fn encode_shortcopy(&mut self, best_match: Match) {
        assert!(best_match.length >= 2 && best_match.length < 6);
        assert!(best_match.offset >= -0x100 && best_match.offset < 0);
        self.encode_control(0);
        self.encode_control(0);
        self.encode_control((((best_match.length - 2) >> 1) & 1) as u32);
        self.encode_control(((best_match.length - 2) & 1) as u32);
        self.buffer.push(best_match.offset as u8);
    }

    fn encode_longcopy(&mut self, best_match: Match) {
        assert!(best_match.length >= 3 && best_match.length < 0x101);
        assert!(best_match.offset >= -0x1FFF && best_match.offset < 0);
        self.encode_control(0);
        self.encode_control(1);
        if best_match.length > 9 {
            self.buffer.push((best_match.offset << 3) as u8);
            self.buffer.push((best_match.offset >> 5) as u8);
            self.buffer.push((best_match.length - 1) as u8);
        }
        else {
            self.buffer.push((best_match.offset << 3) as u8 | (best_match.length - 2) as u8);
            self.buffer.push((best_match.offset >> 5) as u8);
        }
    }

    fn encode_finish(&mut self) {
        self.encode_control(0);
        self.encode_control(1);
        self.buffer.push(0);
        self.buffer.push(0);
   }

    fn encode_control(&mut self, bit: u32) {
        assert!(bit < 2);
        // If control byte is filled, make a new one
        if self.control_byte_bit == 8 {
            // Reset control byte index
            self.control_byte_idx = self.buffer.len();
            // Push an empty control byte
            self.buffer.push(0);
            // Reset current bit
            self.control_byte_bit = 0;
        }
        // Push new bit
        let control_byte = self.buffer.get_mut(self.control_byte_idx).unwrap();
        *control_byte |= (bit as u8) << self.control_byte_bit;
        self.control_byte_bit += 1;
    }
}
