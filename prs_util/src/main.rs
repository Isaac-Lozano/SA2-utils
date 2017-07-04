extern crate prs_util;

use std::fs::File;
use std::io::Write;
use std::env;

use prs_util::Decoder;

fn main() {
    let mut args = env::args().skip(1);
    let filename = args.next().unwrap();
    let out_filename = args.next().unwrap();

    let file = File::open(filename).unwrap();
    let mut decoder = Decoder::new(file);
    let decoded = decoder.decode_to_vec().unwrap();

    let mut out = File::create(out_filename).unwrap();
    out.write_all(&decoded).unwrap();
}
