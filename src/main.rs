#![feature(never_type)]

use aoc_2021::*;
use std::{fs, time::Instant};

fn run_solution<T: Solution>(solution: T) {
    let name = solution.name();
    let raw_input = fs::read_to_string(format!("./data/{}.txt", name))
        .expect(&format!("Could not find input for {}", name));

    let part_one_start = Instant::now();
    let part_one_answer = solution.part_one(&solution.parse(&raw_input));
    let part_one_duration = Instant::now().duration_since(part_one_start).as_micros();
    println!("{}a: {} ({}us)", name, part_one_answer, part_one_duration);

    let part_two_start = Instant::now();
    let part_two_answer = solution.part_two(&solution.parse(&raw_input));
    let part_two_duration = Instant::now().duration_since(part_two_start).as_micros();
    println!("{}b: {} ({}us)", name, part_two_answer, part_two_duration);
}

fn main() {
    run_solution(day_1::Solution);
    run_solution(day_2::Solution);
    // run_solution(day_3::Solution);
    run_solution(day_4::Solution);
}
