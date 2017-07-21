extern crate byteorder;
extern crate radx;

use std::env;
use std::fs::File;

use byteorder::{BigEndian, WriteBytesExt};

fn main() {
    let mut args = env::args().skip(1);
    let filename = args.next().unwrap();

    let f = File::open(filename).unwrap();
    let adx = radx::from_reader(f).unwrap();

    println!("channels: {}", adx.channels());
    println!("Sample rate: {}", adx.sample_rate());

    let mut file = File::create("output.i16be").unwrap();

    for sample in adx {
        file.write_i16::<BigEndian>(sample[0]).unwrap();
        file.write_i16::<BigEndian>(sample[1]).unwrap();
    }
}
