use std::collections::HashSet;

use tracing::warn;

mod parse;
pub use parse::parse_str;

mod machine;
use machine::Machine;

#[derive(Debug)]
struct State {
    max_depth: usize,
    solutions: HashSet<Vec<usize>>,
    target: Vec<u16>,
}

fn recurse(
    state: &mut State,
    buttons: &[Vec<u16>],
    sums: &[u16],
    solution: &mut Vec<usize>,
    current_depth: usize,
) {
    if current_depth > state.max_depth {
        return;
    }

    let mut all_equal = true;

    for (t, s) in state.target.iter().zip(sums) {
        if s > t {
            return;
        } else if s != t {
            all_equal = false;
        }
    }

    if all_equal {
        // debug!(?state, ?sums, ?solution, current_depth);
        if current_depth < state.max_depth {
            state.max_depth = current_depth;
            state.solutions.insert(solution[..current_depth].to_vec());
        }
        return;
    }

    for (idx, button) in buttons.iter().enumerate() {
        let new_sum: Vec<u16> = button.iter().zip(sums).map(|(i, s)| *i + *s).collect();

        if solution.len() < current_depth + 1 {
            solution.push(idx);
        } else {
            solution[current_depth] = idx;
        }

        recurse(state, buttons, &new_sum, solution, current_depth + 1);
    }
}

pub fn main(machines: Vec<Machine>) -> usize {
    let mut result = 0usize;

    for machine in machines.into_iter() {
        let mut state = State {
            max_depth: 50,
            solutions: HashSet::new(),
            target: machine.joltage,
        };

        let sums = vec![0u16; machine.width];

        let mut solution = Vec::new();

        recurse(&mut state, &machine.masks, &sums, &mut solution, 0);

        result += state.max_depth;

        warn!(?state);
    }

    result
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use super::parse::parse_str;
    use super::*;

    fn init_tracing() {
        tracing_subscriber::fmt()
            .compact()
            .with_target(false)
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .with_timer(tracing_subscriber::fmt::time::ChronoLocal::new(
                "%H:%M:%S%.3f".to_owned(),
            ))
            .init();
    }

    #[test]
    fn one() {
        init_tracing();

        let s = "
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        ";

        let machines = parse_str(s);

        let result = main(machines);

        assert_eq!(33, result);
    }

    #[test]
    fn with_input() {
        init_tracing();

        let s = read_to_string("/home/miguel/novaera/rust/aoc2025/src/ten/input.txt")
            .expect("could not read ten/input.txt");

        let machines = parse_str(&s);

        let result = main(machines);

        dbg!(result);
    }
}
