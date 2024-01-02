use std::{collections::HashSet, borrow::{Borrow, BorrowMut}};

use crate::engine;


struct Gear {
    nums: HashSet<usize>
}

impl Gear {
    fn new() -> Self {
        Gear { nums: HashSet::new() }
    }

    fn is_valid(&self) -> bool {
        self.nums.len() == 2
    }

    fn insert(&mut self, val: &Option<usize>) {
        if let Some(val) = val {
            self.nums.insert(val.clone());
        }
    }

    fn product(&self) -> usize {
        self.nums.iter().copied().reduce(|acc, element| acc * element).unwrap_or(0).clone()
    }
}

pub fn calculate_gear_ratio(engine: &engine::Engine) -> usize {
    let mut gears: Vec<Gear> = Vec::new();
    for cell in engine.possible_gears().iter() {
        if cell.is_possible_gear() {
            let mut gear = Gear::new();
            // current row
            get_numbers_for_gear_at_row(engine, gear.borrow_mut(), cell.row(), cell.col());
            // above
            get_numbers_for_gear_at_row(engine, gear.borrow_mut(), cell.row()-1, cell.col());
            // below
            get_numbers_for_gear_at_row(engine, gear.borrow_mut(), cell.row()+1, cell.col());
            if gear.is_valid() {
                gears.push(gear)
            }
        }
    }

    let mut ratio = 0;
    for gear in gears {
        ratio += gear.product()
    }

    ratio
}

fn get_numbers_for_gear_at_row(engine: &engine::Engine, gear: &mut Gear, row: usize, center: usize) {
    // center
    gear.insert(engine.number_at(row, center).borrow());

    // left
    if center > 0 {
        gear.insert(engine.number_at(row, center-1).borrow());
    }

    // right
    gear.insert(engine.number_at(row, center+1).borrow());
}


#[cfg(test)]
mod test_gear {
    use crate::engine;
    use super::calculate_gear_ratio;
    
    #[test]
    fn test_sample() {
        // arrange
        let mut engine = engine::Engine::new();
        let samle: [&str; 10] = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."
        ];

        for s in &samle {
            _ = engine.add_line(s)
        }

        // act
        let ratio = calculate_gear_ratio(&engine);

        // assert
        assert_eq!(467835, ratio)
    }
}
