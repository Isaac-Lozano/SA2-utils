extern crate encoding;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::iter::Peekable;
use std::slice;

use encoding::{Encoding, EncoderTrap, DecoderTrap};
use encoding::codec::japanese::Windows31JEncoding;

#[derive(Clone,Copy,Debug,PartialEq,Eq,Serialize,Deserialize)]
pub enum Language {
    Japanese,
    English,
    French,
    Spanish,
    German,
}

#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
pub enum TextElement {
    Sound(u32),
    Wait(u32),
    D,
    Text(String),
}

impl TextElement {
    pub fn is_meta(&self) -> bool {
        match *self {
            TextElement::Text(_) => false,
            _ => true,
        }
    }
}

#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
pub struct Sa2Text {
    pub language: Language,
    pub elements: Vec<TextElement>,
}

impl Sa2Text {
    pub fn to_bytes(&self) -> Vec<u8> {
        enum State {
            Text,
            Meta,
        }
        let mut state = None;
        let mut bytes = Vec::new();

        for element in self.elements.iter() {
            match element {
                e if element.is_meta() => {
                    match state {
                        Some(State::Text) | None => bytes.push(0x0c),
                        _ => {}
                    }
                    state = Some(State::Meta);

                    match *e {
                        TextElement::Sound(num) => {
                            bytes.push('s' as u8);
                            let num_string = num.to_string().into_bytes();
                            bytes.extend_from_slice(&num_string);
                        }
                        TextElement::Wait(num) => {
                            bytes.push('w' as u8);
                            let num_string = num.to_string().into_bytes();
                            bytes.extend_from_slice(&num_string);
                        }
                        TextElement::D => {
                            bytes.push('D' as u8);
                        }
                        _ => unreachable!(),
                    }
                }
                &TextElement::Text(ref string) => {
                    match state {
                        Some(State::Meta) => bytes.extend_from_slice(&[0x20, 0x07]),
                        None => bytes.push(0x07),
                        _ => {}
                    }
                    state = Some(State::Text);

                    // XXX: Do some proper checks on codepoints and actual errors.
                    match self.language {
                        Language::Japanese => {
                            let encoding = Windows31JEncoding;
                            bytes.extend_from_slice(&encoding.encode(&string, EncoderTrap::Strict).unwrap());
                        }
                        _ => {
                            bytes.extend(string.chars().map(|c| c as u32 as u8));
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        bytes
    }

    pub fn from_slice(slice: &[u8], language: Language) -> Sa2Text {
        let mut elements = Vec::new();
        let mut peeker = slice.iter().peekable();

        loop {
            match peeker.next() {
                Some(&0x0c) => {
                    let mut meta_data = Sa2Text::read_meta(&mut peeker);
                    elements.append(&mut meta_data);
                }
                Some(&0x07) => {
                    let text = TextElement::Text(Sa2Text::read_text(&mut peeker, language));
                    elements.push(text);
                }
                None => return Sa2Text { elements: elements, language: language},
                _ => panic!("Bad command byte."),
            }
        }
    }

    fn read_meta(peeker: &mut Peekable<slice::Iter<u8>>) -> Vec<TextElement> {
        let mut elements = Vec::new();

        loop {
            match peeker.next() {
                // 's'
                Some(&0x73) => {
                    let num = Sa2Text::read_number(peeker);
                    elements.push(TextElement::Sound(num));
                }
                // 'w'
                Some(&0x77) => {
                    let num = Sa2Text::read_number(peeker);
                    elements.push(TextElement::Wait(num));
                }
                // 'D'
                Some(&0x44) => {
                    elements.push(TextElement::D);
                }
                // ' '
                Some(&0x20) | None => return elements,
                _ => panic!("Bad meta byte."),
            }
        }
    }

    fn read_number(peeker: &mut Peekable<slice::Iter<u8>>) -> u32 {
        let mut num = 0;

        loop {
            match peeker.peek().map(|x| (**x as char).is_digit(10)) {
                Some(true) => {
                    num *= 10;
                    num += (peeker.next().map(|x| *x).unwrap() as char).to_digit(10).unwrap();
                }
                _ => return num,
            }
        }
    }

    fn read_text(peeker: &mut Peekable<slice::Iter<u8>>, language: Language) -> String {
        let mut str_data = Vec::new();
        loop {
            match peeker.peek() {
                Some(&&0x0c) | None => {
                    match language {
                        // Japanese uses SHIFT JIS encoding
                        Language::Japanese => {
                            let encoding = Windows31JEncoding;
                            return encoding.decode(&str_data, DecoderTrap::Strict).unwrap();
                        }
                        // Everything else uses Latin1
                        _ => {
                            // Fancy stuff because Latin1 and UTF-8 codepoints match up
                            return str_data.into_iter().map(|c| c as char).collect();
                        }
                    }
                }
                Some(_) => str_data.push(*peeker.next().unwrap()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
