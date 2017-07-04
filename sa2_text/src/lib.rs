use std::iter::Peekable;
use std::slice;

#[derive(Clone,Debug)]
pub enum TextElement {
    Sound(u32),
    Wait(u32),
    D,
    Text(String),
}

#[derive(Clone,Debug)]
pub struct Sa2Text {
    text: Vec<TextElement>,
}

impl Sa2Text {
    pub fn from_slice(slice: &[u8]) -> Sa2Text {
        let mut elements = Vec::new();
        let mut peeker = slice.iter().peekable();

        loop {
            match peeker.next() {
                Some(&0x0c) => {
                    let mut meta_data = Sa2Text::read_meta(&mut peeker);
                    elements.append(&mut meta_data);
                }
                Some(&0x07) => {
                    let text = TextElement::Text(Sa2Text::read_text(&mut peeker));
                    elements.push(text);
                }
                None => return Sa2Text { text: elements },
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

    fn read_text(peeker: &mut Peekable<slice::Iter<u8>>) -> String {
        let mut str_data = Vec::new();
        loop {
            match peeker.peek() {
                Some(&&0x0c) | None => return String::from_utf8(str_data).unwrap(),
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
