use day_02::part1::process_input;

fn main() {
    let file = include_str!("../../input2.txt");
    let result = process_input(file);
    println!("{}", result);
}
