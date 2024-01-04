mod card;
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
    let mut cards: Vec<card::Card> = Vec::new();

    while let Some(t) = r.next() {
        if !t.is_empty() {
            if let Ok(card) = card::Card::new(t.as_str()) {
                cards.push(card);
            }
        }
    }

    let mut points = 0;
    for card in cards.iter() {
        points += card.calculate_points();
    }

    println!("Part One: points = {}", points);
    println!("Part Two: total scratch cards = {}", card::calculate_number_of_scratch_cards(cards));
}
