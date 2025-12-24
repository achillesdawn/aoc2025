use std::collections::HashSet;
use tracing::{debug, debug_span, warn};

mod parse;

mod machine;
use machine::Machine;

fn process_machine(machine: Machine) -> usize {
    // check if steps are 0 or 1
    if machine.state == 0 {
        return 0;
    } else if machine.masks.contains(&machine.state) {
        return 1;
    }

    let mut steps = 2usize;
    let mut results: HashSet<u16> = HashSet::from_iter(machine.masks.clone());

    const MAX_DEPTH: usize = 10usize;
    let mut found = false;

    let span = debug_span!("proc", target = machine.state);
    let _guard = span.enter();

    warn!("start");

    for _ in 0..MAX_DEPTH {
        let new_results: Vec<u16> = machine
            .masks
            .iter()
            .flat_map(|button| {
                let temp: Vec<u16> = results.iter().map(|v| *button ^ *v).collect();
                temp
            })
            .collect();

        if new_results.contains(&machine.state) {
            found = true;
            break;
        } else {
            steps += 1;
            results.extend(new_results);
        }

        debug!(?results);
    }

    if !found {
        panic!("not found in MAX_STEPS");
    }

    debug!(?results, steps);

    steps
}

pub fn main(machines: Vec<Machine>) -> usize {
    let mut result = 0usize;

    for machine in machines.into_iter() {
        result += process_machine(machine);
    }

    result
}

#[cfg(test)]
mod tests {

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

        for machine in machines.iter() {
            println!("{}", machine);
        }

        assert_eq!(7, main(machines));
    }
}
