extern crate byteorder;
extern crate radx;

use std::env;
use std::io::{self, Read};
use std::fs::File;

use byteorder::{BigEndian, ReadBytesExt};

use radx::AdxSpec;
use radx::encoder::standard_encoder::StandardEncoder;

fn main() {
    let mut args = env::args().skip(1);
    let filename = args.next().unwrap();

    let mut input = File::open(filename).unwrap();
    let output = File::create("output.adx").unwrap();

    let spec = AdxSpec {
        channels: 2,
        sample_rate: 44100,
        loop_info: None,
    };
    let mut encoder = StandardEncoder::new(output, spec).unwrap();
    println!("{:#?}", encoder);

    println!("Reading Samples.");
    let mut samples = Vec::new();
    while let Ok((sample1, sample2)) = read_sample(&mut input) {
        let sample = vec![sample1, sample2];
        samples.push(sample);
    }

    println!("Encoding data.");
    encoder.encode_data(samples).unwrap();
    encoder.finish().unwrap();
}

fn read_sample<R>(mut reader: R) -> io::Result<(i16, i16)>
    where R: Read
{
    let s1 = reader.read_i16::<BigEndian>()?;
    let s2 = reader.read_i16::<BigEndian>()?;
    Ok((s1, s2))
}