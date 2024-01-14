use crate::{read_file_to_string, SolveAdvent};
use std::collections::{HashMap, HashSet};

pub struct Day4;

impl SolveAdvent for Day4 {
    fn solve_part1(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let mut total_score_of_cards = 0;
        for line in file_as_str.lines() {
            let (_card_number, your_numbers, winning_numbers) = process_card(line);
            let winning_numbers = winning_numbers.into_iter().collect::<HashSet<_>>();
            let mut winning_card_count = 0;
            for your_number in your_numbers {
                if winning_numbers.contains(&your_number) {
                    winning_card_count += 1;
                }
            }
            total_score_of_cards += score_card(winning_card_count);
        }
        println!("Total Cards Score: {}", total_score_of_cards);
    }

    fn solve_part2(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        //The card_counter HashMap stores how many of each card you have won. Intialize it to all to 1's
        let mut card_counter = HashMap::new();
        for i in 1..file_as_str.lines().count() + 1 {
            card_counter.insert(i, 1);
        }
        card_counter.insert(1, 1);
        for line in file_as_str.lines() {
            let (card_number, your_numbers, winning_numbers) = process_card(line);
            let winning_numbers = winning_numbers.into_iter().collect::<HashSet<_>>();
            let mut winning_card_count = 0;
            for your_number in your_numbers {
                if winning_numbers.contains(&your_number) {
                    winning_card_count += 1;
                }
            }
            //Get how many cards of this number we have won (including duplicates won)
            let card_count = card_counter.get(&card_number).unwrap().clone();

            //For each of the next winning_card_count cards, we win an extra card_count of that card.
            for card_number in card_number + 1..card_number + winning_card_count + 1 {
                *card_counter.get_mut(&card_number).unwrap() += card_count;
            }
        }
        let sum_of_cards: i32 = card_counter.values().map(|item| item.clone()).sum();
        println!("Total Card Counts including duplicates: {}", sum_of_cards);
    }
}

fn process_card(line: &str) -> (usize, Vec<usize>, Vec<usize>) {
    //! Given a line representing a single card, process the card into 3 parts,
    //! the card number, the winning numbers, and the numbers you have.
    let mut line_split = line.split(':');
    let card_number = line_split
        .next()
        .unwrap()
        .replace("Card ", "")
        .trim()
        .parse::<usize>()
        .unwrap();
    let mut all_numbers_split = line_split.next().unwrap().split('|');
    let winning_numbers = all_numbers_split
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .filter(|item| item.len() > 0)
        .map(|number| number.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let your_numbers = all_numbers_split
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .filter(|item| item.len() > 0)
        .map(|number| number.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    (card_number, your_numbers, winning_numbers)
}

fn score_card(winning_card_count: usize) -> usize {
    //! As the instructions describe, the first winning card scores 1, each
    //! subsequent winning card doubles the score.
    if winning_card_count == 0 {
        return 0;
    }
    let mut score = 1;
    for _ in 1..winning_card_count {
        score *= 2;
    }
    score
}
