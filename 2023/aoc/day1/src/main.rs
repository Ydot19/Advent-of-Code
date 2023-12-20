pub extern crate clap;
use clap::Parser;
pub(crate) mod reader;
pub(crate) mod calibration;
pub(crate) mod trie;

#[derive(Parser, Debug)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let mut r = reader::FileReader::new(args.path.as_path().to_str().unwrap());

    let mut sum: i32 = 0;
    loop {
        match r.read_line_calibration_value() {
            Some(n) => {
                sum += n;
            },
            None => break,
        }
    }

    println!("Calibration value: {}", sum);
}
