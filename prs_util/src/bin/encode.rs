extern crate prs_util;

use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use std::env;

use prs_util::encoder::Encoder;

fn main() {
    let mut args = env::args().skip(1);
    let filename = args.next().unwrap();
    let out_filename = args.next().unwrap();

    let mut file = BufReader::new(File::open(filename).unwrap());
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let encoder = Encoder::new(&buf);
    let encoded = encoder.encode();

    let mut out = BufWriter::new(File::create(out_filename).unwrap());
    out.write_all(&encoded).unwrap();
}
