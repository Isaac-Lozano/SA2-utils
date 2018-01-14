extern crate afs_util;

use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::env;

use afs_util::AfsReader;

fn main() {
    let mut args = env::args().skip(1);
    let filename = args.next().unwrap();

    let file = BufReader::new(File::open(filename).unwrap());
    let mut afs = AfsReader::new(file).unwrap();
    let len = afs.len();

    for idx in 0..len {
        let mut test = afs.open(idx).unwrap().unwrap();
        println!("Extracting file {} out of {}", idx + 1, len);
        let mut out = BufWriter::new(File::create(format!("adx/{}.adx", idx)).unwrap());
        io::copy(&mut test, &mut out).unwrap();
    }
}
