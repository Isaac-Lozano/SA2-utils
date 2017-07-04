extern crate sa2_set;
extern crate serde_json;

use std::env;
use std::fs::File;

use sa2_set::{SetFile, Dreamcast, GameCube, Pc};

fn main() {
    let mut args = env::args().skip(1);
    match args.next().unwrap().as_str() {
        "-decode" => {
            let mut set_file = File::open(args.next().unwrap()).expect("Could not open set file.");
            let set_objs = SetFile::from_read::<GameCube, _>(&mut set_file).expect("Could not parse set file.");

            let json_file = File::create(args.next().unwrap()).expect("Could not create json file.");
            serde_json::to_writer_pretty(json_file, &set_objs).expect("Could not write json data.");
        }
        "-encode" => {
            let json_file = File::open(args.next().unwrap()).expect("Could not open json file.");
            let set_objs = serde_json::from_reader::<_, SetFile>(json_file).expect("Could not parse json file.");

            let mut set_file = File::create(args.next().unwrap()).expect("Could not create set file.");
            set_objs.write_data::<GameCube, _>(&mut set_file).expect("Could not write set data.");
        }
        _ => panic!("Bad action flag."),
    }
}
