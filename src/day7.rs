use crate::{read_file_to_string, SolveAdvent};
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Day7;

impl SolveAdvent for Day7 {
    fn solve_part1(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let mut processed_hands = file_as_str
            .lines()
            .map(|line| {
                let mut line_splitter = line.split(" ");
                let hand = line_splitter.next().unwrap().trim();
                let wager = line_splitter
                    .next()
                    .unwrap()
                    .trim()
                    .parse::<usize>()
                    .unwrap();
                PokerHand::new(hand, wager)
            })
            .collect::<Vec<_>>();
        //Because Ord trait is implemented for PokerHand, we can just sort the vec.
        processed_hands.sort();

        let mut total_winnings = 0;
        for (rank, poker_hand) in processed_hands.iter().enumerate() {
            total_winnings += (rank + 1) * poker_hand.wager;
        }
        println!("Total Winnings: {}", total_winnings);
    }

    fn solve_part2(path_to_file: &str) {
        let _ = path_to_file;
        todo!();
    }
}

#[derive(Eq, PartialEq, Debug)]
struct PokerHand {
    hand_name: HandTypes,
    poker_rank: usize,
    hand: String,
    wager: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum HandTypes {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

fn map_char_to_num(card: char) -> usize {
    //! If the card is a face card, return a corresponding
    //! number to make numerical comparison easier.
    if card == 'A' {
        return 14;
    } else if card == 'K' {
        return 13;
    } else if card == 'Q' {
        return 12;
    } else if card == 'J' {
        return 11;
    } else if card == 'T' {
        return 10;
    }
    //If the card is a digit, convert to usize and return.
    const RADIX: u32 = 10;
    card.to_digit(RADIX).unwrap() as usize
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.poker_rank == other.poker_rank && self.hand == other.hand {
            return Some(Ordering::Equal);
        }
        //Because ranks start at 0 and count up, a higher poker_rank in our designation is bad.
        if self.poker_rank > other.poker_rank {
            return Some(Ordering::Less);
        } else if self.poker_rank < other.poker_rank {
            return Some(Ordering::Greater);
        }
        //If the two hands are of the same rank, then compare each char one-by-one to determine which hand is better.
        for (char1, char2) in self.hand.chars().zip(other.hand.chars()) {
            let char1_as_num = map_char_to_num(char1);
            let char2_as_num = map_char_to_num(char2);
            if char1_as_num > char2_as_num {
                return Some(Ordering::Greater);
            } else if char2_as_num > char1_as_num {
                return Some(Ordering::Less);
            }
        }
        //In theory, this should be unreachable.
        Some(Ordering::Equal)
    }
}

impl Ord for PokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        //! Partial ORD implementation is already good enough
        self.partial_cmp(other).unwrap()
    }
}

fn char_count(hand: &str) -> Vec<usize> {
    //! Returns a Vec of the count of each char in the hand.
    //! This can be used to deduce the type of hand present.
    let mut char_counter: HashMap<char, usize> = HashMap::with_capacity(hand.len());
    for char in hand.chars() {
        *char_counter.entry(char).or_default() += 1;
    }
    char_counter
        .values()
        .map(|item| item.clone())
        .collect::<Vec<_>>()
}

impl PokerHand {
    fn new(hand: &str, wager: usize) -> PokerHand {
        //! Given the hand, figure out which Poker hand the hand represents,
        //! and returns a PokerHand struct.
        let char_count = char_count(hand);
        let (hand_name, rank) = if char_count.contains(&5) {
            //Five of a kind
            (HandTypes::FiveOfAKind, 1)
        } else if char_count.contains(&4) {
            //4 of a kind
            (HandTypes::FourOfAKind, 2)
        } else if char_count.contains(&3) && char_count.contains(&2) {
            //Full house
            (HandTypes::FullHouse, 3)
        } else if char_count.contains(&3) {
            //Three of a kind
            (HandTypes::ThreeOfAKind, 4)
        } else if char_count.iter().filter(|item| item == &&2_usize).count() == 2 {
            //Two pair
            (HandTypes::TwoPair, 5)
        } else if char_count.contains(&2) {
            //Pair
            (HandTypes::Pair, 6)
        } else {
            //High Card
            (HandTypes::HighCard, 7)
        };
        PokerHand {
            hand_name,
            poker_rank: rank,
            hand: hand.to_owned(),
            wager,
        }
    }
}
