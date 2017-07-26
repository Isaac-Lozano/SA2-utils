extern crate sa2_text;
extern crate prs_util;
extern crate serde_json;

use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::PathBuf;
use std::process;

use sa2_text::{Sa2TextTable, Language};
use prs_util::encoder::Encoder;
use prs_util::decoder::Decoder;

#[cfg(windows)]
const NEWLINE: &'static [u8] = b"\r\n";
#[cfg(not(windows))]
const NEWLINE: &'static [u8] = b"\n";

fn main() {
    let mut args = env::args().skip(1);

    let input: PathBuf = args.next().unwrap_or_else(|| barf("Error while reading arguments", HintEditorError("no input file specified"))).into();
    match input.extension() {
        Some(os_str) => {
            match os_str.to_str() {
                Some("json") => {
                    let output = input.with_extension("prs");

                    let json_file = File::open(&input).unwrap_or_else(|e| barf("Error while opening json file", e));
                    let hint_table = serde_json::from_reader::<_, Sa2TextTable>(json_file).unwrap_or_else(|e| barf("Error while parsing json file", e));

                    let mut text_table_buffer = Vec::new();
                    hint_table.to_writer(&mut text_table_buffer).unwrap_or_else(|e| barf("Error while writing hint data", e));
                    let encoder = Encoder::new(&text_table_buffer);
                    let encoded = encoder.encode();

                    let mut hint_file = File::create(output).unwrap_or_else(|e| barf("Error while creating hint file", e));
                    hint_file.write_all(&encoded).unwrap_or_else(|e| barf("Error while writing hint file", e));
                    println!("Successfully encoded file");
                }
                Some("prs") => {
                    let output = input.with_extension("json");

                    let mut hint_file = File::open(&input).unwrap_or_else(|e| barf("Error while opening hint file", e));
                    let mut prs_buffer = Vec::new();
                    hint_file.read_to_end(&mut prs_buffer).unwrap_or_else(|e| barf("Error while reading hint file", e));

                    let mut decoder = Decoder::new(prs_buffer.as_slice());
                    let decoded = decoder.decode_to_vec().unwrap_or_else(|e| barf("Error while reading decoding file", e));

                    let mut prs_cursor = Cursor::new(decoded);
                    let hint_table = Sa2TextTable::from_seek(&mut prs_cursor, Language::English).unwrap_or_else(|e| barf("Error while reading hint file", e));

                    let mut json_file = File::create(output).unwrap_or_else(|e| barf("Error while creating json file", e));
                    let mut first = true;
                    json_file.write_all(b"{").unwrap_or_else(|e| barf("Error while writing json file", e));
                    json_file.write_all(NEWLINE).unwrap_or_else(|e| barf("Error while writing json file", e));
                    json_file.write_all(b"  \"language\": ").unwrap_or_else(|e| barf("Error while writing json file", e));
                    serde_json::to_writer(&mut json_file, &hint_table.language).unwrap_or_else(|e| barf("Error while writing json file", e));
                    json_file.write_all(b",").unwrap_or_else(|e| barf("Error while writing json file", e));
                    json_file.write_all(NEWLINE).unwrap_or_else(|e| barf("Error while writing json file", e));
                    json_file.write_all(b"  \"texts\": [").unwrap_or_else(|e| barf("Error while writing json file", e));
                    json_file.write_all(NEWLINE).unwrap_or_else(|e| barf("Error while writing json file", e));
                    for hint in hint_table.texts {
                        if !first {
                            json_file.write_all(b",").unwrap_or_else(|e| barf("Error while writing json file", e));
                            json_file.write_all(NEWLINE).unwrap_or_else(|e| barf("Error while writing json file", e));
                        }
                        else {
                            first = false;
                        }

                        json_file.write_all(b"    ").unwrap_or_else(|e| barf("Error while writing json file", e));
                        serde_json::to_writer(&mut json_file, &hint).unwrap_or_else(|e| barf("Error while writing json file", e));
                    }
                    json_file.write_all(NEWLINE).unwrap_or_else(|e| barf("Error while writing json file", e));
                    json_file.write_all(b"  ]").unwrap_or_else(|e| barf("Error while writing json file", e));
                    json_file.write_all(NEWLINE).unwrap_or_else(|e| barf("Error while writing json file", e));
                    json_file.write_all(b"}").unwrap_or_else(|e| barf("Error while writing json file", e));

                    println!("Successfully decoded file");
                }
                _ => barf("Error", HintEditorError("incorrect extension (expected \".bin\" or \".json\")")),
            }
        }
        _ => barf("Error", HintEditorError("incorrect extension (expected \".bin\" or \".json\")")),
    }
}

fn barf<E>(intro: &str, err: E) -> !
    where E: Error
{
    println!("{}: {}", intro, err);
    process::exit(1);
}

#[derive(Copy,Clone,Debug)]
struct HintEditorError(&'static str);

impl fmt::Display for HintEditorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for HintEditorError {
    fn description(&self) -> &str {
        "HintEditorError"
    }
}
