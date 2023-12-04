use regex::Regex;

struct Card {
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
    copy_count: u32,
}

fn parser(s: String) -> Vec<Card> {
    let re = Regex::new(r"(\d+)").unwrap();
    let cards: Vec<Card> = s
        .split('\n')
        .map(|card_str| {
            let (win, mine) = card_str.split_once(':').unwrap().1.split_once('|').unwrap();
            Card {
                winning_numbers: re
                    .find_iter(win)
                    .map(|m| m.as_str().parse::<u32>().unwrap())
                    .collect(),
                my_numbers: re
                    .find_iter(mine)
                    .map(|m| m.as_str().parse::<u32>().unwrap())
                    .collect(),
                copy_count: 0,
            }
        })
        .collect();
    cards
}

fn solve(cards: &Vec<Card>) -> u32 {
    cards
        .iter()
        .map(|card| {
            let matches: Vec<&u32> = card
                .winning_numbers
                .iter()
                .filter(|win_num| card.my_numbers.contains(&win_num))
                .collect();
            if matches.len() == 0 {
                0
            } else {
                matches.iter().skip(1).fold(1, |a, _| a * 2)
            }
        })
        .sum()
}

fn solve2(cards: &mut Vec<Card>) -> u32 {
    for card_ndx in 0..cards.len() {
        let cur_card = &cards[card_ndx];
        let matches: Vec<&u32> = cur_card
            .winning_numbers
            .iter()
            .filter(|win_num| cur_card.my_numbers.contains(&win_num))
            .collect();
        let this_card_copies = cur_card.copy_count + 1;
        for copy_card_ndx in card_ndx+1..(card_ndx+1+matches.len()) {
            let copy_card = &mut cards[copy_card_ndx];
            copy_card.copy_count += 1 * this_card_copies;
        }
    }
    cards
        .iter()
        .fold(0, |a, c| {
            a + c.copy_count + 1
        })
}

fn main() {
    let mut scratch_cards = utils::load_puzzle_data(4, parser);
    let total_points = solve(&scratch_cards);
    println!("Solution 1: Total scratch card points: {}", total_points);

    let total_num_cards = solve2(&mut scratch_cards);
    println!(
        "Solution 1: Total number of scratch cards: {}",
        total_num_cards
    );
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(4, 1, parser);
        let solution = solve(&test_data);
        assert_eq!(solution, 13);
    }

    #[test]
    fn test_puzzle2() {
        let mut test_data = utils::load_puzzle_test(4, 1, parser);
        let solution = solve2(&mut test_data);
        assert_eq!(solution, 30);
    }
}