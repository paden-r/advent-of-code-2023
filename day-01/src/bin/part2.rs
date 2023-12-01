use std::collections::HashMap;

fn main() {
    let input = include_str!("./day-01-input.txt");
    let total = part_2(input);
    println!("{}", total);
}

fn word_to_number_conversion(word: &str) -> &str{
    let word_to_number = HashMap::new();
    let mut new_word = "";
    for char in word.chars() {
        new_word = format!("{}{}", new_word, char).as_str();
        match new_word.contains()
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