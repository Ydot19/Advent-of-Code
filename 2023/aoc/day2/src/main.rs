mod game;
pub extern crate clap;
use std::borrow::Borrow;

use clap::Parser;
use utils;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short = 'p', long)]
    path: std::path::PathBuf,
    #[clap(long)]
    green: i32,
    #[clap(long)]
    red: i32,
    #[clap(long)]
    blue: i32
}

fn main() {
    let args = Cli::parse();
    let mut r = utils::reader::FileReader::new(args.path.as_path().to_str().unwrap());

    let mut cube = game::Cube::new();
    cube.insert(game::Color::Blue, args.blue);
    cube.insert(game::Color::Red, args.red);
    cube.insert(game::Color::Green, args.green);

    let mut sum_of_possible_games: i32 = 0;
    let mut sum_of_power_of_sets: i32 = 0;
    let color_slice = &[game::Color::Red, game::Color::Green, game::Color::Blue];
    while let Some(t) = r.next() {
        let g = game::Game::new(t.as_str());
        if g.is_possible(cube.borrow()) {
            sum_of_possible_games += g.number()
        }

        let possible_cubes = g.max_number_of_cubes_by_color_across_sets();
        let mut product: i32 = 1;
        for el in color_slice.iter() {
            product *= possible_cubes.get_value_by_color(el.clone())
        }
        
        sum_of_power_of_sets += product
    }

    println!("Sum of Possible Games: {}", sum_of_possible_games);
    println!("Sum of Product of Minimum Color Set: {}", sum_of_power_of_sets)
}
