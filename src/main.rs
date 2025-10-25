use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let sum = nums[0] + nums[1];
    println!("{}", sum);
}
