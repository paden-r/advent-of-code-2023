fn main() {
    let input = include_str!("day-01-input.txt");
    let total = part_1(input);
    println!("{}", total);
}

fn part_1(input: &str) -> u32 {
    let mut total = 0;
    for word in input.split('\n') {
        let mut first_number = 0;
        let mut last_number = 0;
        let mut first_number_found = false;
        for char in word.chars() {
            if char.is_numeric() {
                if !first_number_found {
                    first_number = char.to_digit(10).unwrap();
                    last_number = char.to_digit(10).unwrap();
                    first_number_found = true;
                } else {
                    last_number = char.to_digit(10).unwrap();
                }
            }
        }
        let number_string = format!("{}{}", first_number, last_number);
        total += number_string.parse::<u32>().unwrap();
        first_number = 0;
        last_number = 0
    }
    total
}

fn part_1_method_2(input: &str) -> u32 {
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars();

            let first = it
                .find_map(|character| {
                    character.to_digit(10)
                })
                .expect("should be a number");

            let last = it
                .rfind(|character| character.is_digit(10))
                .map(|character| {
                    character.to_digit(10).unwrap()
                })
                // if we don't find a number, then we're
                // re-using the first number
                .unwrap_or(first);

            first * 10 + last
        })
        .sum::<u32>();
    output
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn passing_test() {
        let test_input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let actual = part_1(test_input);
        assert_eq!(actual, 142);
    }
}