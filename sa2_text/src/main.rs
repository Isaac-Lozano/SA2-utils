extern crate sa2_text;
extern crate strable;

use std::env;
use std::fs::File;

use sa2_text::Sa2Text;
use strable::Strable;

fn main() {
    let mut args = env::args().skip(1);
    let filename = args.next().unwrap();

    let mut f = File::open(filename).unwrap();
    let mut strable = Strable::from_seek(&mut f).unwrap();

    for (idx, data) in strable.strings().enumerate() {
        println!("data: {:?}", data);
        println!("idx {}: {:?}", idx, Sa2Text::from_slice(&data));
    }
}
