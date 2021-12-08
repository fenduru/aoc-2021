pub mod solutions;

use solutions::day_1a;
use std::fs;

fn main() {
    let input_1a = fs::read_to_string("./data/1a.txt").expect("Could not read input for 1a");
    println!("Day 1a answer: {}", day_1a::solution(&input_1a));
}
