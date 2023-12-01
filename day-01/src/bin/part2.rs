#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
static ref WORD_TO_NUMBER: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("one", 1);
        m.insert("two", 2);
        m.insert("three", 3);
        m.insert("four", 4);
        m.insert("five", 5);
        m.insert("six", 6);
        m.insert("seven", 7);
        m.insert("eight", 8);
        m.insert("nine", 9);
        m.insert("zero", 0);
        m
    };
}

fn main() {
    let input = include_str!("./day-01-input.txt");
    let total = part_2(input);
    println!("{}", total);
}


fn word_to_number_conversion(word: &str) -> &str {
    let mut new_word = "";
    for char in word.chars() {
        println!("{}", char);
        // new_word = format!("{}{}", new_word, char).as_str();
        // match new_word.contains(WORD_TO_NUMBER.iter())
    }
    new_word
}

fn part_2(input: &str) -> u32 {
    let mut total = 0;
    for word in input.split('\n') {
        let mut first_number = 0;
        let mut last_number = 0;
        for char in word.chars() {
            if char.is_numeric() {
                if first_number == 0 {
                    first_number = char.to_digit(10).unwrap();
                    last_number = char.to_digit(10).unwrap();
                } else {
                    last_number = char.to_digit(10).unwrap();
                }
            }
        }
        match last_number {
            0 => {
                let number_string = format!("{}{}", first_number, first_number);
                total += number_string.parse::<u32>().unwrap();
            }
            _ => {
                let number_string = format!("{}{}", first_number, last_number);
                total += number_string.parse::<u32>().unwrap();
            }
        }
        first_number = 0;
        last_number = 0
    }
    total
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn passing_test() {
        let test_input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let actual = part_2(test_input);
        assert_eq!(actual, 281);
    }
}