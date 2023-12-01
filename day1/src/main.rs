
const NUMBER_WORDS: &[[&str; 2]] = &[
    ["eight", "8"],
    ["five", "5"],
    ["four", "4"],
    ["nine", "9"],
    ["one", "1"],
    ["three", "3"],
    ["two", "2"],
    ["seven", "7"],
    ["six", "6"],
];

fn parser(s: String) -> Vec<String> {
    s.split('\n').map(str::to_string).collect()
}

fn solve(amended_calibrations: &Vec<String>) -> u32 {
    amended_calibrations.iter().map(|s| {
        let mut nums = s.chars().filter_map(|c| c.to_digit(10));
        let first = nums.next().unwrap();
        (first * 10) + nums.nth_back(0).unwrap_or(first)
    }).collect::<Vec<u32>>().iter().sum()
}

fn solve2(amended_calibrations: &Vec<String>) -> u32 {
    // Just replace all of the words with a number, then solve()
    let replaced_calibrations: Vec<String> = amended_calibrations.iter().map(|s|{
        let mut new_string = String::new();
        for n in 0..s.len() {
            match NUMBER_WORDS.iter().find(|&[word, _word_num]| {
                match s.get(n..(n+word.len())) {
                    Some(s_word) => s_word.cmp(word).is_eq(),
                    None => false,
                }
            }) {
                Some(found_number_word) => {
                    new_string.push(found_number_word[1].chars().next().unwrap());
                },
                None => {
                    new_string.push(s.chars().nth(n).unwrap());
                },
            };
        }
        new_string
    }).collect();

    println!("Original: {:?} Replaced: {:?}", amended_calibrations, replaced_calibrations);
    solve(&replaced_calibrations)
}

fn main() {
    let amended_calibrations = utils::load_puzzle_data(1, parser);
    let calibration_value = solve(&amended_calibrations);
    println!(
        "Solution 1: The calibration value is: {}",
        calibration_value
    );

    let calibration_value = solve2(&amended_calibrations);
    println!(
        "Solution 2: The calibration value is: {}",
        calibration_value
    );
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(1, 1, parser);
        let solution = solve(&test_data);
        assert_eq!(solution, 142);

    }

    #[test]
    fn test_puzzle2() {
        let test_data = utils::load_puzzle_test(1, 2, parser);
        let solution = solve2(&test_data);
        assert_eq!(solution, 281);

    }
}