fn main() {
    let input = include_str!("day-01-input.txt");
    let total = part_1(input);
    println!("{}", total);
}

fn part_1(input: &str) -> u32 {
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