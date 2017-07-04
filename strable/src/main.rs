extern crate byteorder;
extern crate strable;

use std::env;
use std::fs::File;

use strable::Strable;

fn main() {
    let mut args = env::args().skip(1);
    let filename = args.next().unwrap();

    let mut f = File::open(filename).unwrap();
    let mut strable = Strable::from_seek(&mut f).unwrap();

    for (idx, data) in strable.strings().enumerate() {
        println!("idx {}: {:?}", idx, String::from_utf8(data.clone().into_iter().map(|x| if x >= 0x80 { 0x21 } else { x }).collect()));
    }

    let mut g = File::create("just_checking.bin").unwrap();
    strable.write_data(&mut g).unwrap();
}
