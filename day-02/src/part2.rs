use lazy_static::lazy_static;
use fancy_regex::Regex;

lazy_static! {
static ref RED_PATTERN: Regex = Regex::new(r"(\d{1}|\d{2})\sred").unwrap();
static ref GREEN_PATTERN: Regex = Regex::new(r"(\d{1}|\d{2})\sgreen").unwrap();
static ref BLUE_PATTERN: Regex = Regex::new(r"(\d{1}|\d{2})\sblue").unwrap();
}

struct CubeGame {
   minimum_cubes_multiplied: u32
}

impl CubeGame {
    pub fn from_str(line: &str) -> Self {
        let red_max = Self::find_largest(&RED_PATTERN, line);
        let green_max = Self::find_largest(&GREEN_PATTERN, line);
        let blue_max = Self::find_largest(&BLUE_PATTERN, line);
        let multi = red_max * green_max * blue_max;
        Self {
           minimum_cubes_multiplied: multi
        }
    }

    pub fn find_largest(pattern: &Regex, line: &str) -> u32 {
        let mut largest: u32 = 0;
        for cap in pattern.captures_iter(line) {
            let number = cap.expect("Error getting pattern").get(1).expect("No group").as_str().parse::<u32>().unwrap();
            if number > largest {
                largest = number;
            }
        }
        largest
    }
}

pub fn process_input(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let this_game = CubeGame::from_str(line);
            this_game.minimum_cubes_multiplied
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
        assert_eq!(actual, 2286);
    }
}
