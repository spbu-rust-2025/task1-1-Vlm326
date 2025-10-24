use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input error");
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Parcing error"))
        .collect();

    let sum = numbers[0] + numbers[1];
    println!("{}", sum);
}
