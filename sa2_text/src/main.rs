extern crate sa2_text;

use std::env;
use std::fs::File;

use sa2_text::Sa2Text;

fn main() {
    let mut args = env::args().skip(1);
    let filename = args.next().unwrap();

    let mut f = File::open(filename).unwrap();
