use std::collections::VecDeque;
use std::{str::FromStr, usize};
use std::cmp::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Numeric,
    GearSymbol,
    OtherSymbol,
    DotSymbol
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub kind: Kind,
    val: char,
    row: usize,
    col: usize
}

impl Cell {
    pub fn new(c: char) -> Cell {
        let mut _k = Kind::OtherSymbol;

        match c {
            '.' => {
                _k = Kind::DotSymbol
            },
            '*' => {
                _k = Kind::GearSymbol
            },
            _ => {
                if c.is_digit(10) {
                    _k = Kind::Numeric
                }
            }
        }

        Cell { 
            kind: _k, 
            val: c,
            row: 0,
            col: 0
         }
    }

    pub fn is_symbol(&self) -> bool {
        match self.kind {
        Kind::OtherSymbol | Kind::GearSymbol => {
            true
        },
        _ => false
        }
    }

    pub fn is_numeric(&self) -> bool {
        match self.kind {
        Kind::Numeric => {
            true
        },
        _ => false
        }
    }

    pub fn is_possible_gear(&self) -> bool {
        match self.kind {
        Kind::GearSymbol => true,
        _ => false
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCellError;

impl FromStr for Cell {
    type Err = ParseCellError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
        1 => {
            Ok(Cell::new(s.chars().nth(0).unwrap()))
        }
        _ => {
            Err(ParseCellError)
        }
        }
    }
}

pub struct Number {
    pub val: usize,
    pub row: usize,
    pub start_index: usize,
    pub end_index: usize,
}



impl Number {
    fn new(val: usize, row: usize, start: usize, end: usize) -> Self {
        Number {
            val: val,
            row: row,
            start_index: start,
            end_index: end,
        }
    }
}

pub struct Engine {
    schema: Vec<Vec<Cell>>
}

impl Engine {
    pub fn new() -> Engine {
        Engine{
            schema: Vec::new()
        }
    }

    pub fn add_line(&mut self, s: &str) -> Result<(), ParseCellError> {
        let mut l = Vec::new();
        let row: usize = self.schema.len();
        for (col, c) in s.chars().enumerate() {
            let result = c.to_string().as_str().parse::<Cell>();
            if result.is_err() {
                return Err(result.unwrap_err());
            } else {
                let mut cell = result.unwrap();
                cell.col = col;
                cell.row = row;
                l.push(cell);
            }
        }

        self.schema.push(l);
        Ok(())
    }

    pub fn numbers(&self) -> Vec<Number> {
        let mut nums: Vec<Number> = Vec::new();
        for (row_number, row) in self.schema.iter().enumerate() {
            let mut curr_number = String::new();
            let mut char_index: usize = 0;
            let mut start = usize::MAX;
            let mut end = usize::MIN;
            for cell in row.iter() {
                if cell.is_numeric() {
                    curr_number.push(cell.val);
                    if char_index < start {
                        start = char_index
                    }
                    end = char_index
                } else {
                    if !curr_number.is_empty() {
                        let val = curr_number.parse::<usize>().unwrap();
                        let num = Number::new(val, row_number, start, end);
                        nums.push(num);
                        start = usize::MAX;
                        end = usize::MIN;
                        curr_number.clear();
                    }
                }

                char_index += 1;
            }

            if !curr_number.is_empty() {
                let val = curr_number.parse::<usize>().unwrap();
                end = char_index-1;
                let num = Number::new(val, row_number, start, end);
                nums.push(num);
            }
        }
        return nums
    }

    pub fn possible_gears(&self) -> Vec<Cell> {
        let mut res = Vec::new();
        for cells in self.schema.iter() {
            for cell in cells.iter() {
                if cell.is_possible_gear() {
                    res.push(cell.clone())
                }
            }
        }

        res
    }

    pub fn number_at(&self, row: usize, col: usize) -> Option<usize> {
        if let Some(row) = self.schema.get(row) {
            if let Some(cell) = row.get(col) {
                if cell.is_numeric() {
                    let mut char_num: VecDeque<char> = VecDeque::new();
                    char_num.push_back(cell.val);
                    let mut right = col;
                    loop {
                        right += 1;
                        let right_cell = row.get(right);
                        if let Some(cell) = right_cell {
                            if cell.is_numeric() {
                                char_num.push_back(cell.val)
                            } else {
                                break
                            }
                        } else {
                            break
                        }
                    }

                    let mut left = col;
                    loop {
                        if left > 0 {
                            left -= 1;
                            let left_cell = row.get(left);
                            if let Some(cell) = left_cell {
                                if cell.is_numeric() {
                                    char_num.push_front(cell.val)
                                } else {
                                    break;
                                }
                            }
                        } else {
                            break
                        }
                    }

                    let num = match char_num.into_iter().collect::<String>().as_str().parse::<usize>() {
                        Ok(v) => Some(v),
                        Err(_) => None
                    };

                    return num
                }
            }
        }
        None
    }



    pub fn is_number_adjacent_to_symbol(&self, num: &Number) -> bool {
        // left
        let mut start = num.start_index;
        if num.start_index > 0 {
            start -= 1;
            if let Some(cell) = self.get_cell(num.row, start) {
                if cell.is_symbol() {
                    return true
                }
            }
        }

        // right
        let end = num.end_index +1;
        if let Some(cell) = self.get_cell(num.row, end) {
            if cell.is_symbol() {
                return true
            }
        }
            
        

        // above
        if num.row > 0 {
            for column in start..end+1 {
                if let Some(cell) = self.get_cell(num.row-1, column) {
                    if cell.is_symbol() {
                        return true
                    }
                }
            }
        }
        
        let above_row: usize = num.row+1;
        if self.get_row_len(above_row).is_some() {
            for column in start..end+1 {
                if let Some(cell) = self.get_cell(above_row, column) {
                    if cell.is_symbol() {
                        return true
                    }
                }
            }
        }

        false
    }

    fn get_cell(&self, row: usize, column: usize) -> Option<&Cell> {
        if let Some(cells) = self.schema.get(row) {
            if let Some(cell) = cells.get(column) {
                return Some(cell);
            }
        }

        None
    }

    fn get_row_len(&self, row: usize) -> Option<usize> {
        if let Some(cells) = self.schema.get(row) {
            return Some(cells.len())
        }

        None
    }
}

#[cfg(test)]
mod test_cell {
    use super::Cell;

    #[test]
    fn test_numeric_char() {
        // arrange
        const C: char = '9';
        // act
        let cell = Cell::new(C);

        // assert
        assert_eq!(C, cell.val);
        assert!(cell.is_numeric());
    }

    #[test]
    fn test_symbol_char() {
        // arrange
        const C: char = '#';
        // act
        let cell = Cell::new(C);

        // assert
        assert_eq!(C, cell.val);
        assert!(cell.is_symbol());
    }

    #[test]
    fn test_dot_char() {
        // arrange
        const C: char = '.';
        // act
        let cell = Cell::new(C);

        // assert
        assert_eq!(C, cell.val);
        assert!(!cell.is_symbol());
        assert!(!cell.is_numeric());
    }
}

#[cfg(test)]
mod test_engine {
    use super::*;

    #[test]
    fn test_string_with_no_numbers() {
        // arrange
        const S: &str = "...*......";
        let mut engine = Engine::new();
        let res = engine.add_line(S);
        assert!(res.is_ok());

        // act
        let nums = engine.numbers();

        // assert
        assert_eq!(0, nums.len())
    }

    #[test]
    fn test_string_with_multiple_numbers() {
        // arrange
        const S: &str = ".664.598..";
        let mut engine = Engine::new();
        let res = engine.add_line(S);
        assert!(res.is_ok());

        // act
        let nums = engine.numbers();

        // assert
        assert_eq!(2, nums.len())
    }

    #[test]
    fn test_string_with_number() {
        // arrange
        const S: &str = ".......58";

        let mut engine = Engine::new();
        let res = engine.add_line(S);
        assert!(res.is_ok());

        // act
        let nums = engine.numbers();

        // assert
        assert_eq!(1, nums.len())
    }

    #[test]
    fn test_sample() {
        // arrange
        let mut engine = Engine::new();
        let sample: [&str; 10] = [
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

        for s in &sample {
            _ = engine.add_line(s)
        }

        let nums = engine.numbers();

        // act
        let valid_nums = nums.into_iter().filter(|n| engine.is_number_adjacent_to_symbol(n));

        // assert
        let expected = 4361;
        let mut actual = 0;
        for num in valid_nums {
            actual += num.val
        }

        assert_eq!(expected, actual)
    }


    #[test]
    fn test_number_at() {
        // arrange
        let mut engine = Engine::new();
        let samle: [&str; 11] = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
            "..91....91"
        ];

        for s in &samle {
            _ = engine.add_line(s)
        }

        // act
        let case_a = engine.number_at(0, 0); // expect 467
        let case_b = engine.number_at(0, 1); // expect 467
        let case_c = engine.number_at(0, 2); // expect 467

        let case_d = engine.number_at(1, 3); // no numbers in row
        let case_e = engine.number_at(99, 0); // out of bounds row
        let case_f = engine.number_at(0, 99); // out of bounds row

        let case_g = engine.number_at(9, 3); // expect 664
        let case_h = engine.number_at(9, 6); // expect 598
        let case_i = engine.number_at(10, 9); // expect 91 - right edge

        // assert
        assert_eq!(Some(467), case_a);
        assert_eq!(Some(467), case_b);
        assert_eq!(Some(467), case_c);

        assert_eq!(None, case_d);
        assert_eq!(None, case_e);
        assert_eq!(None, case_f);

        assert_eq!(Some(664), case_g);
        assert_eq!(Some(598), case_h);
        assert_eq!(Some(91), case_i);
    }
}


