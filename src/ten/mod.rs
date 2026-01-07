mod parse;
pub use parse::parse_str;

mod machine;
use machine::Machine;

// fn recurse(state: &mut State, states: , current_depth: usize) {
//     if states.contains(&state.target) {
//         warn!(current_depth, "found");

//         dbg!(states);

//         state.max_depth = current_depth;

//         return;
//     }

//     debug!(current_depth, states_len = states.len(), "level");

//     let states = states
//         .into_iter()
//         .flat_map(|sum_state| {
//             state
//                 .buttons
//                 .iter()
//                 .map(|button| sum_state + *button)
//                 .collect::<Vec<u16>>()
//         })
//         .filter(|s| *s <= state.target)
//         .collect();

//     debug!(?states);

//     recurse(state, states, current_depth + 1);
// }

pub fn solve_machine(machine: Machine) {
    let mut result = [0u16; 10];

    let min = machine.joltage[..machine.size].iter().min();

    let max = machine.joltage[..machine.size].iter().max();

    dbg!((min, max));

    // machine.calculate_state(&[0, 1, 1, 1, 3, 3, 3, 4, 5, 5], &mut result);

    // dbg!(&result[..machine.size]);
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use crate::{init_tracing, ten::solve_machine};

    use super::parse::parse_str;

    #[test]
    fn one() {
        init_tracing();

        let s = "
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        ";

        let machines = parse_str(s);

        for machine in machines.into_iter() {
            solve_machine(machine);
        }
    }

    #[test]
    fn two() {
        let s = "
[.#.#] (0,2,3) (1,3) (2,3) (0,1,2) (0) {31,4,31,29}
[#..#..##.#] (1,2,3,4,5,6,7,8,9) (2,5,6,7) (0,1,3,5,7,8) (0,2,3,5,6,8,9) (0,1,3,5,6,7,8,9) (4,7) (3,5,7) (4,6) (1,2,4) (0,1,2,4,5,7,8,9) {34,50,61,55,68,80,58,88,50,48}
        ";

        let machines = parse_str(s);

        for machine in machines.into_iter() {}
    }

    #[test]
    fn with_input() {
        init_tracing();

        let s = read_to_string("/home/miguel/novaera/rust/aoc2025/src/ten/input.txt")
            .expect("could not read ten/input.txt");

        let machines = parse_str(&s);

        let max = machines.iter().max_by_key(|m| m.size);

        dbg!(max);
    }
}
