#![feature(never_type)]

mod day_1;

use aoc_2021::Solution;
use std::fs;

fn run_solution<T: Solution>(solution: T) {
    let name = solution.name();
    let raw_input = fs::read_to_string(format!("./data/{}.txt", name))
        .expect(&format!("Could not find input for {}", name));
    let parsed_input = solution.parse(&raw_input);

    let part_one_answer = solution.part_one(&parsed_input);
    println!("{}a: {}", name, part_one_answer);

    let part_two_answer = solution.part_two(&parsed_input);
    println!("{}b: {}", name, part_two_answer);
}

fn main() {
    run_solution(&day_1::Solution);
}
