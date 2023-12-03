use lazy_static::lazy_static;
use fancy_regex::Regex;

const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

lazy_static! {
static ref GAME_PATTERN: Regex = Regex::new(r"Game\s(\d{1}|\d{2}|\d{3}):").unwrap();
static ref RED_PATTERN: Regex = Regex::new(r"(\d{1}|\d{2})\sred").unwrap();
static ref GREEN_PATTERN: Regex = Regex::new(r"(\d{1}|\d{2})\sgreen").unwrap();
static ref BLUE_PATTERN: Regex = Regex::new(r"(\d{1}|\d{2})\sblue").unwrap();
}

struct CubeGame {
    game_number: u32,
    is_valid: bool,
}

impl CubeGame {
    pub fn from_str(line: &str) -> Self {
        let mut is_valid = true;
        let game = GAME_PATTERN.captures(line);
        let captures = game.expect("Error running game regex").expect("No match found");
        let game_number = captures.get(1).expect("No group").as_str().parse::<u32>().unwrap();

        is_valid = Self::check_valid(&RED_PATTERN, RED_MAX, line);
        if is_valid {
            is_valid = Self::check_valid(&GREEN_PATTERN, GREEN_MAX, line);
        }
        if is_valid {
            is_valid = Self::check_valid(&BLUE_PATTERN, BLUE_MAX, line);
        }
        Self {
            game_number,
            is_valid,
        }
    }

    pub fn check_valid(pattern: &Regex, max: u32, line: &str) -> bool {
        let mut is_valid = true;
        for cap in pattern.captures_iter(line) {
            let number = cap.expect("Error getting pattern").get(1).expect("No group").as_str().parse::<u32>().unwrap();
            if number > max {
                is_valid = false;
                break;
            }
        }
        is_valid
    }
}

pub fn process_input(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let this_game = CubeGame::from_str(line);
            match this_game.is_valid {
                true => this_game.game_number,
                false => 0
            }
        })
        .sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let test_input = include_str!("../test_input2.txt");
        let actual = process_input(test_input);
        assert_eq!(actual, 8);
    }
}
