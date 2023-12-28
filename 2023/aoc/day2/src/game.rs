use std::collections::HashMap;
use regex::Regex;
use std::str::FromStr;
use lazy_static::lazy_static;
use std::borrow::Borrow;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Color {
    Red,
    Blue,
    Green,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "blue" => Ok(Color::Blue),
            "green" => Ok(Color::Green),
            _ => Err(()),
        }
    }

}
pub struct Cube {
    colors: HashMap<Color, i32>,
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            colors: HashMap::new(),
        }
    }

    pub fn insert(&mut self, color: Color, count: i32) {
        self.colors.insert(color, count);
    }

    pub fn get_value_by_color(&self, color: Color) -> i32 {
        if let Some(val) = self.colors.get(color.borrow()) {
            return val.clone();
        }

        return 0
    }
}

pub struct Game {
    number: i32,
    sets: Vec<Cube>,
}

impl Game {
    pub fn new(record: &str) -> Game {
        let mut g = Game {
            number: 0,
            sets: Vec::new(),
        };

        g.parse(record);
        g
    }

    pub fn number(&self) -> i32 {
        self.number
    }

    fn parse(&mut self, s: &str ) {        
        // split string s and get the number
        let split_s: Vec<&str>= s.split(":").collect();
        if split_s.len() != 2 {
            panic!("Invalid input string: {}", s);
        }

        let _num = RE.find(split_s[0]).unwrap().as_str().parse::<i32>().unwrap();
        self.number = _num;

        // split string s and get the sets
        let mut split_sets = split_s[1].split(";");
        while let Some(set) = split_sets.next() {
            let mut game_set = Cube::new();
            let mut split_set = set.split(",");
            while let Some(color) = split_set.next() {
                let mut count_colour = color.split_ascii_whitespace();
                let count = count_colour.next().unwrap().trim().parse::<i32>().unwrap();
                let color = count_colour.next().unwrap().parse::<Color>().unwrap();
                game_set.insert(color, count);
            }
            self.sets.push(game_set);
        }
    }

    pub fn is_possible(&self, available_cubes: &Cube) -> bool {
        let mut result = true;
        for set in &self.sets {
            for (color, count) in &set.colors {
                if available_cubes.colors.contains_key(color) {
                    let available_count = available_cubes.colors.get(color).unwrap();
                    if available_count < count {
                        result = false;
                        break;
                    }
                } else {
                    result = false;
                    break;
                }
            }
        }
        result
    }

    pub fn max_number_of_cubes_by_color_across_sets(&self) -> Cube {
        let mut result = Cube::new();
        result.insert(Color::Red, 0);
        result.insert(Color::Blue, 0);
        result.insert(Color::Green, 0);
        
        for set in &self.sets {
            for (color, count) in &set.colors {
                if result.colors.contains_key(color) {
                    let current_count = result.colors.get(color).unwrap();
                    if current_count < count {
                        result.insert(*color, *count);
                    }
                } else {
                    result.insert(*color, *count);
                }
            }
        }

        result
    }
}


#[cfg(test)]
mod test_cube {
    use super::*;

    #[test]
    fn test_cube_new() {
        let c = Cube::new();
        assert_eq!(c.colors.len(), 0);
    }

    #[test]
    fn test_cube_insert() {
        let mut c = Cube::new();
        c.insert(Color::Red, 1);
        assert_eq!(c.colors.len(), 1);
        assert_eq!(c.colors.get(&Color::Red).unwrap(), &1);
    }
}

#[cfg(test)]
mod test_color {
    use super::*;

    #[test]
    fn test_color_from_str() {
        assert_eq!(Color::from_str("red").unwrap(), Color::Red);
        assert_eq!(Color::from_str("blue").unwrap(), Color::Blue);
        assert_eq!(Color::from_str("green").unwrap(), Color::Green);
    }
}

#[cfg(test)]
mod test_game{
    use std::borrow::Borrow;

    use super::*;

    #[test]
    fn test_game_new() {
        // arrange
        let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        // assert
        let g = Game::new(s);

        // assert
        assert_eq!(1, g.number);
        assert_eq!(3,  g.sets.len());
        assert_eq!(Some(i32::from(3)), g.sets[0].colors.get(Color::Blue.borrow()).copied());
        assert_eq!(Some(i32::from(4)), g.sets[0].colors.get(Color::Red.borrow()).copied());
        assert_eq!(None, g.sets[0].colors.get(Color::Green.borrow()))
    }

    #[test]
    fn test_max_number_of_cubes_by_color_across_sets() {
        // arrange
        let s = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let g = Game::new(s);
        assert_eq!(i32::from(3), g.number);

        // act
        let res = g.max_number_of_cubes_by_color_across_sets();
        
        //
        assert_eq!(i32::from(6), res.colors.get(Color::Blue.borrow()).copied().unwrap());
        assert_eq!(i32::from(20), res.colors.get(Color::Red.borrow()).copied().unwrap());
        assert_eq!(i32::from(13), res.colors.get(Color::Green.borrow()).copied().unwrap());
    }

    #[test]
    fn test_is_possible_false() {
        // arrange
        let s = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let g = Game::new(s);
        assert_eq!(i32::from(3), g.number);

        let mut c = Cube::new();
        c.colors.insert(Color::Blue, i32::from(3)); // not enough
        c.colors.insert(Color::Green, i32::from(32));
        c.colors.insert(Color::Red, i32::from(99));

        // act
        let result = g.is_possible(c.borrow());

        // assert
        assert!(!result)
    }

    #[test]
    fn test_is_possible_true() {
        // arrange
        let s = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let g = Game::new(s);
        assert_eq!(i32::from(3), g.number);

        let mut c = Cube::new();
        c.colors.insert(Color::Blue, i32::from(39));
        c.colors.insert(Color::Green, i32::from(32));
        c.colors.insert(Color::Red, i32::from(99));

        // act
        let result = g.is_possible(c.borrow());

        // assert
        assert!(result)
    }
}