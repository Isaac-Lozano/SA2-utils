extern crate prs_util;

use std::fs::File;
use std::io::{Write, BufReader, BufWriter};
use std::env;

use prs_util::decoder::Decoder;

fn main() {
    let mut args = env::args().skip(1);
    let filename = args.next().unwrap();
    let out_filename = args.next().unwrap();

    let file = BufReader::new(File::open(filename).unwrap());
    let mut decoder = Decoder::new(file);
    let decoded = decoder.decode_to_vec().unwrap();

    let mut out = BufWriter::new(File::create(out_filename).unwrap());
    out.write_all(&decoded).unwrap();
}
