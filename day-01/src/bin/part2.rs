use std::collections::HashMap;

fn main() {
    let input = include_str!("./day-01-input.txt");
    let total = part_2(input);
    println!("{}", total);
}

fn word_to_number_conversion(input_word: String) -> u32 {
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
    let mut iterate = (0..input_word.len()).filter_map(|index| {
        let reduced_line = &input_word[index..];
        // println!("{}", reduced_line);
        let mut result = reduced_line.chars().next().unwrap().to_digit(10);
        for (word, number) in &m {
            if reduced_line.starts_with(word){
                result = Some(*number)
            }
        }
        result
    });

    let first = iterate.next().unwrap();
    // println!("{}", first);
    match iterate.last() {
        Some(num) => first * 10 + num,
        None => first * 10 + first,
    }
}

fn part_2(input: &str) -> u32 {
    let mut total = 0;
    for word in input.split('\n') {
        total += word_to_number_conversion(word.to_string());
    }
    total
}

fn part_2_method_2(input: &str) -> u32 {
    let output =
        input.lines().map(process_words).sum::<u32>();
    output
}

fn process_words(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            Some(1)
        } else if reduced_line.starts_with("two") {
            Some(2)
        } else if reduced_line.starts_with("three") {
            Some(3)
        } else if reduced_line.starts_with("four") {
            Some(4)
        } else if reduced_line.starts_with("five") {
            Some(5)
        } else if reduced_line.starts_with("six") {
            Some(6)
        } else if reduced_line.starts_with("seven") {
            Some(7)
        } else if reduced_line.starts_with("eight") {
            Some(8)
        } else if reduced_line.starts_with("nine") {
            Some(9)
        } else {
            reduced_line
                .chars()
                .next()
                .unwrap()
                .to_digit(10)
        };

        result
    });
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => first * 10 + num,
        None => first * 10 + first,
    }
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