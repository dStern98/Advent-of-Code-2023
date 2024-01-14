use crate::{read_file_to_string, SolveAdvent};
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Day7;

const AVAILABLE_CARDS: [char; 12] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

impl SolveAdvent for Day7 {
    fn solve_part1(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let mut processed_hands = process_into_poker_hands(file_as_str, false);
        //Because Ord trait is implemented for PokerHand, we can just sort the vec.
        processed_hands.sort();

        let mut total_winnings = 0;
        for (rank, poker_hand) in processed_hands.iter().enumerate() {
            total_winnings += (rank + 1) * poker_hand.wager;
        }
        println!("Total Winnings: {}", total_winnings);
    }

    fn solve_part2(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let mut poker_hands = process_into_poker_hands(file_as_str, true);
        for hand in poker_hands.iter_mut() {
            //For each hand, compute the best possible HandType using the wildcards.
            //Reset the hand_type field to this new optimized hand_type.
            let hand_type_with_wildcard = hand.compute_wildcard_handtype();
            hand.hand_type = hand_type_with_wildcard;
        }

        // Because Ord trait is implemented for PokerHand, we can just sort the vec.
        poker_hands.sort();

        let mut total_winnings = 0;
        for (rank, poker_hand) in poker_hands.iter().enumerate() {
            total_winnings += (rank + 1) * poker_hand.wager;
        }
        println!("Total Winnings: {}", total_winnings);
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct PokerHand {
    hand_type: HandType,
    hand: String,
    wager: usize,
    ///use_wildcard field is required to communicate to the Ord and PartialOrd
    /// impl whether or not to count the J as a 1 or a 11 when comparing card-by-card
    use_wildcard: bool,
}

///Enum representing all possible Poker hand types in this game.
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Copy)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

impl PokerHand {
    fn compute_wildcard_handtype(&self) -> HandType {
        //! Recursively explore different PokerHand options by replacing
        //! the J's (the wildcard) with a different card.

        //If the hand passed does not contain a J, then no more recursion is required.
        if !self.hand.contains(&'J'.to_string()) {
            return self.hand_type;
        }
        let mut optional_hands = Vec::new();
        for card in AVAILABLE_CARDS {
            let new_hand_with_wildcard = self.hand.replacen('J', &card.to_string(), 1);
            let new_poker_hand = PokerHand::new(&new_hand_with_wildcard, self.wager, true);
            //Recursively compute the next wildcard replacement optimization.
            let optional_hand = new_poker_hand.compute_wildcard_handtype();
            optional_hands.push(optional_hand);
        }
        //Return the minimum handtype possible, which is the best hand possible.
        optional_hands.into_iter().min().unwrap()
    }
}

fn process_into_poker_hands(file_as_str: String, use_wildcard: bool) -> Vec<PokerHand> {
    //! Iterate over the input file, building each hand into a `PokerHand` type.
    file_as_str
        .lines()
        .map(|line| {
            let mut line_splitter = line.split(' ');
            let hand = line_splitter.next().unwrap().trim();
            let wager = line_splitter
                .next()
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();
            PokerHand::new(hand, wager, use_wildcard)
        })
        .collect::<Vec<_>>()
}

fn map_card_to_num(card: char, use_wildcards: bool) -> usize {
    //! If the card is a face card, return a corresponding
    //! number to make numerical comparison easier.
    //! As described in the problem directions, if wildcards are counted,
    //! then wildcards count as 1, not the usual 11.
    if card == 'A' {
        return 14;
    } else if card == 'K' {
        return 13;
    } else if card == 'Q' {
        return 12;
    } else if card == 'J' && !use_wildcards {
        return 11;
    } else if card == 'J' && use_wildcards {
        return 1;
    } else if card == 'T' {
        return 10;
    }
    //If the card is a digit, convert to usize and return.
    const RADIX: u32 = 10;
    card.to_digit(RADIX).unwrap() as usize
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        //! Partial ORD implementation is already good enough
        if self.hand_type == other.hand_type && self.hand == other.hand {
            return Ordering::Equal;
        }
        //Because ranks start at 0 and count up, a higher hand_type in our designation is bad.
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Greater => {
                return Ordering::Less;
            }
            Ordering::Less => {
                return Ordering::Greater;
            }
            Ordering::Equal => {}
        }
        //If the two hands are of the same rank, then compare each char one-by-one to determine which hand is better.
        for (char1, char2) in self.hand.chars().zip(other.hand.chars()) {
            let char1_as_num = map_card_to_num(char1, self.use_wildcard);
            let char2_as_num = map_card_to_num(char2, other.use_wildcard);
            match char1_as_num.cmp(&char2_as_num) {
                Ordering::Greater => {
                    return Ordering::Greater;
                }
                Ordering::Less => {
                    return Ordering::Less;
                }
                Ordering::Equal => {}
            }
        }
        Ordering::Equal
    }
}

fn char_count(hand: &str) -> Vec<usize> {
    //! Returns a Vec of the count of each char in the hand.
    //! This can be used to deduce the type of hand present.
    let mut char_counter: HashMap<char, usize> = HashMap::with_capacity(hand.len());
    for char in hand.chars() {
        *char_counter.entry(char).or_default() += 1;
    }
    char_counter.values().copied().collect::<Vec<_>>()
}

impl PokerHand {
    fn new(hand: &str, wager: usize, use_wildcard: bool) -> PokerHand {
        //! Given the hand, figure out which Poker hand the hand represents,
        //! and returns a `PokerHand`
        let char_count = char_count(hand);
        let hand_type = if char_count.contains(&5) {
            //Five of a kind
            HandType::FiveOfAKind
        } else if char_count.contains(&4) {
            //4 of a kind
            HandType::FourOfAKind
        } else if char_count.contains(&3) && char_count.contains(&2) {
            //Full house
            HandType::FullHouse
        } else if char_count.contains(&3) {
            //Three of a kind
            HandType::ThreeOfAKind
        } else if char_count.iter().filter(|item| item == &&2_usize).count() == 2 {
            //Two pair
            HandType::TwoPair
        } else if char_count.contains(&2) {
            //Pair
            HandType::Pair
        } else {
            //High Card
            HandType::HighCard
        };
        PokerHand {
            hand_type,
            hand: hand.to_owned(),
            wager,
            use_wildcard,
        }
    }
}
