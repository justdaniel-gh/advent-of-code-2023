#[derive(Debug)]
struct Hand {
    reds: u32,
    greens: u32,
    blues: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
}

fn parser(s: String) -> Vec<Game> {
    s.split('\n')
        .enumerate()
        .map(|(n, g)| Game {
            id: n as u32 + 1,
            hands: g
                .split_once(':')
                .unwrap()
                .1
                .split(';')
                .map(|a_hand| {
                    let colors = a_hand.split(", ");
                    let mut reds = 0;
                    let mut blues = 0;
                    let mut greens = 0;
                    for color in colors {
                        let color = color.trim().split_once(' ').unwrap();
                        match color.1 {
                            "blue" => {
                                blues += color.0.parse::<u32>().unwrap();
                            }
                            "red" => {
                                reds += color.0.parse::<u32>().unwrap();
                            }
                            "green" => {
                                greens += color.0.parse::<u32>().unwrap();
                            }
                            _ => (),
                        }
                    }
                    Hand {
                        reds,
                        greens,
                        blues,
                    }
                })
                .collect(),
        })
        .collect()
}

fn solve(games: &Vec<Game>) -> u32 {
    // We have a list of Games, now to find out whats in them...
    // For this solution we want to know:
    //  Which games are possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
    // Then return the sum of the ids

    // If any game has less than or equal to 12 red, 13 green, and 14 blue in any one hand, it's possible
    let possible_games: Vec<&Game> = games
        .into_iter()
        .filter(|g| {
            g.hands
                .iter()
                .all(|h| h.reds <= 12 && h.greens <= 13 && h.blues <= 14)
        })
        .collect();
    possible_games.iter().fold(0, |a, g| a + g.id)
}

fn solve2(games: &Vec<Game>) -> u32 {
    // For this solution we want to know:
    //  What is the fewest number of cubes of each color that could have been in the bag to make the game possible?
    // Then return the sum of the power of the sets

    // Power =  reds * greens * blues

    // Find the largest value of each color, in all hands, for each game
    // We'll return a Hand object to store the values
    let max_colors_per_game: Vec<Hand> = games
        .into_iter()
        .map(|g| {
            g.hands.iter().fold(
                Hand {
                    reds: 0,
                    blues: 0,
                    greens: 0,
                },
                |mut acc, a_hand| {
                    if a_hand.reds > acc.reds {
                        acc.reds = a_hand.reds;
                    }
                    if a_hand.blues > acc.blues {
                        acc.blues = a_hand.blues;
                    }
                    if a_hand.greens > acc.greens {
                        acc.greens = a_hand.greens;
                    }
                    acc
                },
            )
        })
        .collect();
    max_colors_per_game
        .iter()
        .fold(0, |a, h| a + (h.reds * h.blues * h.greens))
}

fn main() {
    let games = utils::load_puzzle_data(2, parser);
    let possible_game_sum = solve(&games);
    println!("Solution 1: Sum of possible games: {}", possible_game_sum);

    let possible_game_power_sum = solve2(&games);
    println!(
        "Solution 2: Sum of powers of possible games: {}",
        possible_game_power_sum
    );
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(2, 1, parser);
        let solution = solve(&test_data);
        assert_eq!(solution, 8);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = utils::load_puzzle_test(2, 2, parser);
        let solution = solve2(&test_data);
        assert_eq!(solution, 2286);
    }
}
