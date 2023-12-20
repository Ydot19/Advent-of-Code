#![allow(dead_code)]

use std::collections::HashMap;
use crate::trie::TrieNode;

pub(crate) fn calibration_value(s: &str) -> Option<i32> {
    let n = s.len();
    let mut t: TrieNode = TrieNode::new();
    let mut left: Option<i32> = None;
    for i in 0..n {
        let s1 = String::from(&s[0..i]);
        t.insert(s1.chars().rev().collect::<String>().as_str());
        let r = found_word_number(t.clone(), word_to_number(true));
        if r.is_some() {
            left = r;
            break;
        }

        match s.chars().nth(i) {
            Some(c) => {
               let c_num: Option<i32> = c.to_string().parse().ok();
                if c_num.is_some() {
                     left = c_num;
                     break;
                }
            },
            None => (),
        }
    }


    let mut right: Option<i32> = None;
    for i in (0..n).rev() {
        let s1 = String::from(&s[i..n]);
        t.insert(s1.as_str());
        let r = found_word_number(t.clone(), word_to_number(false));
        if r.is_some() {
            right = r;
            break;
        }

        match s.chars().nth(i) {
            Some(c) => {
               let c_num: Option<i32> = c.to_string().parse().ok();
                if c_num.is_some() {
                     right = c_num;
                     break;
                }
            },
            None => (),
        }
    }

    let mut result: Option<i32> = None;
    if left.is_some() && right.is_some() {
        result = format!("{}{}", left.unwrap(), right.unwrap()).parse().ok();
    }

    result
}

fn found_word_number(t: TrieNode, prefix_terms:  HashMap<String, i32>) -> Option<i32> {
    for (key, value) in prefix_terms {
        if t.prefix(key.clone().as_str()).is_some()  {
            return Some(value);
        }
    }

    None
}

fn word_to_number(reverse: bool) -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();
    
    let mut one = String::from("one");
    let mut two = String::from("two");
    let mut three = String::from("three");
    let mut four = String::from("four");
    let mut five = String::from("five");
    let mut six = String::from("six");
    let mut seven = String::from("seven");
    let mut eight = String::from("eight");
    let mut nine = String::from("nine");

    if reverse {
        one = one.chars().rev().collect::<String>();
        two = two.chars().rev().collect::<String>();
        three = three.chars().rev().collect::<String>();
        four = four.chars().rev().collect::<String>();
        five = five.chars().rev().collect::<String>();
        six = six.chars().rev().collect::<String>();
        seven = seven.chars().rev().collect::<String>();
        eight = eight.chars().rev().collect::<String>();
        nine = nine.chars().rev().collect::<String>();
        
    }

    map.insert(one, 1);
    map.insert(two, 2);
    map.insert(three, 3);
    map.insert(four, 4);
    map.insert(five, 5);
    map.insert(six, 6);
    map.insert(seven, 7);
    map.insert(eight, 8);
    map.insert(nine, 9);
    map
}


#[cfg(test)]
mod test_calibration {
    use super::*;

    #[test]
    fn test_calibration_value() {
        // arrange
        let s = "1abc2";
        let expected: Option<i32> = Some(12);

        // act
        let actual = calibration_value(s);

        // assert
        assert_eq!(expected, actual)
    }

    #[test]
    fn case_b() {
        // arrange
        let s = "pqr3stu8vwx";
        let expected: Option<i32> = Some(38);

        // act
        let actual = calibration_value(s);

        // assert
        assert_eq!(expected, actual)
    }

    #[test]
    fn case_c() {
        // arrange
        let s = "a1b2c3d4e5f";
        let expected: Option<i32> = Some(15);

        // act
        let actual = calibration_value(s);

        // assert
        assert_eq!(expected, actual)
    }

    #[test]
    fn case_d() {
        // arrange
        let s = "treb7uchet";
        let expected: Option<i32> = Some(77);

        // act
        let actual = calibration_value(s);

        // assert
        assert_eq!(expected, actual)
    }

    #[test]
    fn case_e() {
        // arrange
        let s = "two1nine";
        let expected: Option<i32> = Some(29);

        // act
        let actual = calibration_value(s);

        // assert
        assert_eq!(expected, actual)
    }

    #[test]
    fn case_f() {
        // arrange
        let s = "eightwothree";
        let expected: Option<i32> = Some(83);

        // act
        let actual = calibration_value(s);

        // assert
        assert_eq!(expected, actual)
    }

    #[test]
    fn case_g() {
        // arrange
        let s = "abcone2threexyz";
        let expected: Option<i32> = Some(13);

        // act
        let actual = calibration_value(s);

        // assert
        assert_eq!(expected, actual)
    }

    #[test]
    fn case_h() {
        // arrange
        let s = "xtwone3four";
        let expected: Option<i32> = Some(24);

        // act
        let actual = calibration_value(s);

        // assert
        assert_eq!(expected, actual)
    }

    #[test]
    fn case_i() {
        // arrange
        let s = "4nineeightseven2";
        let expected: Option<i32> = Some(42);

        // act
        let actual = calibration_value(s);

        // assert
        assert_eq!(expected, actual)
    }

    #[test]
    fn case_j() {
        // arrange
        let s = "7pqrstsixteen";
        let expected: Option<i32> = Some(76);

        // act
        let actual = calibration_value(s);

        // assert
        assert_eq!(expected, actual)
    }
}
