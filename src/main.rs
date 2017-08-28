mod solve;

use std::time::Instant;
use solve::{solve, SolveStats};

fn main() {
    let numbers = vec![50, 100, 9, 3, 8, 4];
    let target = 857;
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