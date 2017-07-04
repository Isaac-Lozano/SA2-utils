extern crate afs_util;

use std::fs::File;
use std::io;
use std::env;

use afs_util::Afs;

fn main() {
    let mut args = env::args().skip(1);
    let filename = args.next().unwrap();

    let file = File::open(filename).unwrap();
    let mut afs = Afs::new(file).unwrap();
    let len = afs.len();

    for idx in 0..len {
        let mut test = afs.open(idx).unwrap().unwrap();
        let mut out = File::create(format!("adx/{}.adx", idx)).unwrap();
        io::copy(&mut test, &mut out).unwrap();
    }
}
