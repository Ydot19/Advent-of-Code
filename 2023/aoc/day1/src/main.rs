pub extern crate clap;
use clap::Parser;

use crate::calibration::calibration_value;
pub(crate) mod calibration;
pub(crate) mod trie;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short = 'p', long)]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let mut r = utils::reader::FileReader::new(args.path.as_path().to_str().unwrap());
    let mut sum: i32 = 0;
    while let Some(t) = r.next() {
        if let Some(n) = calibration_value(t.as_str()) {
            sum += n;
        }
    }

    println!("Calibration value: {}", sum);
}
