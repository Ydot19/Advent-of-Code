mod gear;
mod engine;
pub extern crate clap;
use clap::Parser;
use utils;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short = 'p', long)]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let mut r = utils::reader::FileReader::new(args.path.as_path().to_str().unwrap());
    let mut engine = engine::Engine::new();

    while let Some(t) = r.next() {
        if !t.is_empty() {
            _ = engine.add_line(t.as_str())
        }
    }

    let mut sum = 0;
    let nums = engine.numbers();
    let valid_nums = nums.into_iter().filter(|n| engine.is_number_adjacent_to_symbol(n));
    for num in valid_nums {
        sum += num.val;
    }
    println!("Sum of Part Numbers: {}", sum);

    
    let ratio = gear::calculate_gear_ratio(&engine);
    println!("Gear Ratio: {}", ratio);
}
