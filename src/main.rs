#[macro_use]
extern crate clap;

mod solve;

use std::time::Instant;
use clap::ArgMatches;

use solve::{solve, SolveStats};

fn main() {
    let matches: ArgMatches = clap_app!(myapp =>
        (name: "countdown-game")
        (version: "0.1.0")
        (author: "Mattias Buelens <mattias.buelens@gmail.com>")
        (about: "Solves the Numbers round from the Countdown game show")
        (@arg TARGET: -t --target +required +takes_value "Target number")
        (@arg NUMBER: +required +multiple "Numbers to use")
    ).get_matches();

    let target_str = matches.value_of("TARGET").unwrap();
    let target = target_str.parse::<i32>().unwrap();

    let mut numbers: Vec<i32> = Vec::new();
    for number_str in matches.values_of("NUMBER").unwrap() {
        numbers.push(number_str.parse::<i32>().unwrap());
    }

    run_solve(numbers, target);
}

fn run_solve(numbers: Vec<i32>, target: i32) {
    let mut stats = SolveStats::new();

    println!("Numbers: {:?}", numbers);
    println!("Target: {}", target);

    let start = Instant::now();
    let solution = solve(numbers, target, &mut stats);
    let elapsed = start.elapsed();

    println!("Solution: {} = {}", &solution, solution.value());
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
    println!("Stats: {} expanded, {} visited", stats.expanded(), stats.visited());
}