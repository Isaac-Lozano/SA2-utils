extern crate sa2_text;
extern crate strable;

use std::env;
use std::fs::File;

use sa2_text::{Sa2TextTable, Language};

fn main() {
    let mut args = env::args().skip(1);
    let language = args.next().unwrap();
    let filename = args.next().unwrap();

    let decode_language = match language.as_str() {
        "j" => Language::Japanese,
        "e" => Language::English,
        "f" => Language::French,
        "s" => Language::Spanish,
        "g" => Language::German,
        _ => panic!("Bad language specifier"),
    };

    let mut f = File::open(filename).unwrap();
    let mut text_table = Sa2TextTable::from_seek(&mut f, decode_language).unwrap();
    println!("{:?}", text_table);
}
