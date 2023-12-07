#![allow(dead_code)]

use std::collections::HashMap;
use crate::trie::TrieNode;

pub fn calibration_value(s: &str) -> Option<i8> {
    let n = s.len();
    let mut t: TrieNode = TrieNode::new();
    let mut left: Option<i8> = None;
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
               let c_num: Option<i8> = c.to_string().parse().ok();
                if c_num.is_some() {
                     left = c_num;
                     break;
                }
            },
            None => (),
        }
    }


    let mut right: Option<i8> = None;
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
               let c_num: Option<i8> = c.to_string().parse().ok();
                if c_num.is_some() {
                     right = c_num;
                     break;
                }
            },
            None => (),
        }
    }

    let mut result: Option<i8> = None;
    if left.is_some() && right.is_some() {
        result = format!("{}{}", left.unwrap(), right.unwrap()).parse().ok();
    }

    result
}

fn found_word_number(t: TrieNode, prefix_terms:  HashMap<String, i8>) -> Option<i8> {
    for (key, value) in prefix_terms {
        if t.prefix(key.clone().as_str()).is_some()  {
            return Some(value);
        }
    }

    None
}

fn word_to_number(reverse: bool) -> HashMap<String, i8> {
    let mut map: HashMap<String, i8> = HashMap::new();
    
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