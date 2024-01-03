use std::{collections::{HashSet, HashMap}, borrow::Borrow};

pub(crate) struct Card {
    lucky_numbers: HashSet<usize>,
    hand: Vec<usize>
}

impl Card {
    pub(crate) fn new(val: &str) -> Result<Self, ()> {
        let mut split_card = val.trim().split(':');
        split_card.next(); // Card 1: lucy_numbers | hand

        if let Some(card_str) = split_card.next() {
            let v: Vec<&str> = card_str.trim().split("|").collect();
            let mut lucky_numbers = HashSet::new();
            if let Some(lucky_numbers_concatenated) = v.get(0) {
                for str_num in lucky_numbers_concatenated.to_string().split_ascii_whitespace() {
                    if let Ok(num) = str_num.trim().parse::<usize>() {
                        lucky_numbers.insert(num);
                    }
                }
            }

            let mut hand = Vec::new();
            if let Some(hand_concatenated) = v.get(1) {
                for str_num in hand_concatenated.to_string().split_ascii_whitespace() {
                    if let Ok(num) = str_num.trim().parse::<usize>() {
                        hand.push(num);
                    }
                }
            }

            let c = Card{
                lucky_numbers,
                hand,
            };
            return Ok(c)
        }

        Err(())
    }

    pub(crate) fn calculate_points(&self) -> usize {
        let mut power = 0;
        const BASE: u32 = 2;
        for val in self.hand.iter() {
            if self.lucky_numbers.contains(val) {
                power += 1;
            }
        }

        if power > 0 {
            return BASE.pow(power-1) as usize
        }

        return usize::MIN;
    }

    fn number_of_scratch_cards(&self) -> usize {
        let mut matching_cards = usize::MIN;
        for val in self.hand.iter() {
            if self.lucky_numbers.contains(val) {
                matching_cards += 1;
            }
        }

        matching_cards
    }
}

pub(crate) fn calculate_number_of_scratch_cards(cards: Vec<Card>) -> usize {
    let mut additional_card_instances: HashMap<usize, usize> = HashMap::new();
    let mut curr_row: usize = 0;
    let mut total: usize = 0;
    for card in cards.iter() {
        let scratches = card.number_of_scratch_cards();
        total += scratches * (1 + additional_card_instances.get(curr_row.borrow()).unwrap_or(&0));
        for i in curr_row+1..curr_row+1+scratches {
            let next = additional_card_instances.get(i.borrow()).unwrap_or(&0);
            additional_card_instances.insert(i, next+1);
        }
        curr_row += 1;
    }

    return total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_a() {
        // arrange
        let s = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let card = Card::new(s);
        assert!(card.is_ok());

        // act
        let actual = card.unwrap().calculate_points();

        // assert
        assert_eq!(8, actual)
    }

    #[test]
    fn test_points_sample() {
        // arramge
        let strings = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        ];

        let mut cards: Vec<Card> = Vec::new();
        for s in strings {
            let card = Card::new(s);
            assert!(card.is_ok());
            cards.push(card.unwrap());
        }

        // act
        let mut actual = 0;
        for c in cards.iter(){
            actual += c.calculate_points();
        }

        // assert
        assert_eq!(13, actual)
    }

    #[test]
    fn test_scratch_cards_sample() {
        // arramge
        let strings = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        ];

        let mut cards: Vec<Card> = Vec::new();
        for s in strings {
            let card = Card::new(s);
            assert!(card.is_ok());
            cards.push(card.unwrap());
        }

        // act
        let actual = calculate_number_of_scratch_cards(cards);

        // assert
        assert_eq!(30, actual)
    }
}

