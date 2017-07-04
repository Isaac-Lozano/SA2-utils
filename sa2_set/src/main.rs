extern crate sa2_set;

use std::env;
use std::fs::File;

use sa2_set::{SetFile, Dreamcast, GameCube, Pc};

fn main() {
    let mut args = env::args().skip(1);
    let platform = args.next().unwrap();
    let filename = args.next().unwrap();

    let mut f = File::open(filename).unwrap();

    let set = match platform.as_str() {
        "d" => SetFile::from_read::<Dreamcast, _>(&mut f).unwrap(),
        "g" => SetFile::from_read::<GameCube, _>(&mut f).unwrap(),
        "p" => SetFile::from_read::<Pc, _>(&mut f).unwrap(),
        _ => panic!("Bad platform id."),
    };

    let SetFile(objs) = set;

    for (idx, obj) in objs.into_iter().enumerate() {
        println!("obj {}: {:#?}", idx, obj);
    }
}
