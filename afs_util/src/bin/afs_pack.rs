extern crate afs_util;

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::env;

use afs_util::AfsWriter;

struct FileGetter {
    path: PathBuf,
    total_files: usize,
    idx: usize,
}

impl FileGetter {
    fn new<P>(into_path: P) -> FileGetter
        where P: Into<PathBuf>
    {
        let path = into_path.into();

        let mut file_count = 0;
        loop {
            let filename = format!("{}.adx", file_count);
            if !path.join(filename).exists() {
                break;
            }
            file_count += 1;
        }

        FileGetter {
            path: path,
            total_files: file_count,
            idx: 0,
        }
    }
}

impl Iterator for FileGetter {
    type Item = BufReader<File>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.total_files {
            None
        }
        else {
            println!("Packing file {} out of {}", self.idx + 1, self.total_files);
            let filename = format!("{}.adx", self.idx);
            self.idx += 1;
            File::open(self.path.join(filename))
                .ok()
                .map(|f| BufReader::new(f))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.total_files, Some(self.total_files))
    }
}

impl ExactSizeIterator for FileGetter {}

fn main() {
    let mut args = env::args().skip(1);
    let folder_name = args.next().unwrap();
    let output_name = args.next().unwrap();

    let output_file = BufWriter::new(File::create(output_name).unwrap());

    let file_getter = FileGetter::new(folder_name);
    let afs_writer = AfsWriter::new(output_file, file_getter);
    afs_writer.write().unwrap();
}
