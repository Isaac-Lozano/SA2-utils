extern crate sa2_set;
extern crate getopts;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[cfg(feature="gui")]
extern crate gtk;

mod obj_table;
#[cfg(windows)]
mod windows_pretty_formatter;
#[cfg(feature="gui")]
mod gui;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;

use sa2_set::{SetFile, Platform, Dreamcast, GameCube, Pc};
use serde::ser::Serialize;
use serde_json::ser::Serializer;
#[cfg(not(windows))]
use serde_json::ser::PrettyFormatter;
use getopts::Options;

#[cfg(windows)]
const NEWLINE: &'static [u8] = b"\r\n";
#[cfg(windows)]
type Sa2PrettyPrinter = windows_pretty_formatter::WindowsPrettyFormatter<'static>;
#[cfg(not(windows))]
const NEWLINE: &'static [u8] = b"\n";
#[cfg(not(windows))]
type Sa2PrettyPrinter = PrettyFormatter<'static>;

enum Mode {
    Encode,
    Decode,
    Gui,
    Help,
}

fn main() {
    let mut env_args = env::args();
    let program = env_args.next().unwrap();
    let args: Vec<_> = env_args.collect();

    let mut opts = Options::new();
    opts.optflag("d", "decode", "decode a setfile to json format");
    opts.optflag("e", "encode", "encode a json file to setfile format");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("s", "single-line", "write objects on a single line");
    opts.optflag("g", "gui", "run in gui mode");

    let matches = match opts.parse(&args) {
        Ok(m) => m,
        Err(e) => barf(&e.to_string()),
    };

    let mut mode = None;
    let mut single_line = false;

    if matches.opt_present("e") {
        mode = Some(Mode::Encode);
    }

    if matches.opt_present("d") {
        mode = match mode {
            None => Some(Mode::Decode),
            Some(_) => barf("Only one action can be specified."),
        };
    }

    if matches.opt_present("g") {
        mode = Some(Mode::Gui);
    }

    if matches.opt_present("h") {
        mode = Some(Mode::Help);
    }

    if matches.opt_present("s") {
        single_line = true;
    }

    match mode {
        Some(Mode::Encode) => {
            let input: PathBuf = matches.free.get(0).unwrap_or_else(|| barf("No input file specified.")).into();
            let output: PathBuf = matches.free.get(1).unwrap_or_else(|| barf("No output file specified.")).into();
            match encode_file::<GameCube>(&input, &output) {
                Ok(_) => println!("Successfully encoded file."),
                Err(e) => barf(&e.to_string()),
            }
        }
        Some(Mode::Decode) => {
            let input: PathBuf = matches.free.get(0).unwrap_or_else(|| barf("No input file specified.")).into();
            let output: PathBuf = matches.free.get(1).unwrap_or_else(|| barf("No output file specified.")).into();
            match decode_file::<GameCube>(&input, &output, single_line) {
                Ok(_) => println!("Successfully decoded file."),
                Err(e) => barf(&e.to_string()),
            }
        }
        None => {
            let input: PathBuf = matches.free.get(0).unwrap_or_else(|| barf("No input file specified.")).into();
            match input.extension() {
                Some(os_str) => {
                    match os_str.to_str() {
                        Some("json") => {
                            let output = input.with_extension("bin");
                            match encode_file::<GameCube>(&input, &output) {
                                Ok(_) => println!("Successfully encoded file."),
                                Err(e) => barf(&e.to_string()),
                            }
                        }
                        Some("bin") => {
                            let output = input.with_extension("json");
                            match decode_file::<GameCube>(&input, &output, single_line) {
                                Ok(_) => println!("Successfully decoded file."),
                                Err(e) => barf(&e.to_string()),
                            }
                        }
                        _ => barf("Not a json or a set file."),
                    }
                }
                _ => barf("Not a json or a set file."),
            }
        }
        Some(Mode::Help) => {
            print_usage(&program, opts);
            process::exit(0);
        }
        Some(Mode::Gui) => {
            run_gui();
        }
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] (INPUT | (-d | -e) INPUT OUTPUT)", program);
    println!("OnVar's Set Editor ({})", env!("CARGO_PKG_VERSION"));
    print!("{}", opts.usage(&brief));
}

fn barf(err: &str) -> ! {
    println!("Error: {}", err);
    process::exit(1);
}

fn encode_file<P>(input: &Path, output: &Path) -> Result<(), &'static str>
    where P: Platform
{
    let json_file = File::open(input).map_err(|_| "Could not open json file.")?;
    let set_objs = serde_json::from_reader::<_, SetFile>(json_file).map_err(|_| "Could not parse json file.")?;

    let mut set_file = File::create(output).map_err(|_| "Could not create set file.")?;
    set_objs.write_data::<P, _>(&mut set_file).map_err(|_| "Could not write set data.")?;

    Ok(())
}

fn decode_file<P>(input: &Path, output: &Path, single_line: bool) -> Result<(), &'static str>
    where P: Platform
{
    let mut set_file = File::open(input).map_err(|_| "Could not open set file.")?;
    let set_objs = SetFile::from_read::<P, _>(&mut set_file).map_err(|_| "Could not parse set file.")?;

    let mut json_file = File::create(output).map_err(|_| "Could not create json file.")?;

    if single_line {
        let mut first = true;
        json_file.write_all(b"[").map_err(|_| "Could not write json data.")?;
        json_file.write_all(NEWLINE).map_err(|_| "Could not write json data.")?;
        for obj in set_objs.0 {
            if !first {
                json_file.write_all(b",").map_err(|_| "Could not write json data.")?;
                json_file.write_all(NEWLINE).map_err(|_| "Could not write json data.")?;
            }
            else {
                first = false;
            }

            json_file.write_all(b"  ").map_err(|_| "Could not write json data.")?;
            serde_json::to_writer(&mut json_file, &obj).map_err(|_| "Could not write json data.")?;
        }
        json_file.write_all(NEWLINE).map_err(|_| "Could not write json data.")?;
        json_file.write_all(b"]").map_err(|_| "Could not write json data.")?;
    }
    else {
        let mut serializer = Serializer::with_formatter(json_file, Sa2PrettyPrinter::new());
        set_objs.serialize(&mut serializer).map_err(|_| "Could not write json data.")?;
    }

    Ok(())
}

#[cfg(feature="gui")]
fn run_gui() {
    let mut gui = gui::SetEditorGui::new(None);
    gui.run().unwrap_or_else(|_| barf("Could not run gui."));
}

#[cfg(not(feature="gui"))]
fn run_gui() {
    barf("Gui support not compiled in.");
}
