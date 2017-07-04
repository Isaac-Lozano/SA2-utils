extern crate strable;

use std::env;
use std::fs::File;

use strable::Strable;

fn main() {
    let mut args = env::args().skip(1);
    let filename = args.next().unwrap();

    let mut f = File::open(filename).unwrap();
    let strable = Strable::from_seek(&mut f).unwrap();
    let data_table = strable.into_vec().into_iter().map(|_| b"\x0cD \x07HEY, A HINT TEXT.".to_vec()).collect();
    let new_strable = Strable::from_vec(data_table);
    let mut g = File::create("output.bin").unwrap();
    new_strable.write_data(&mut g).unwrap();
}
