use arrayvec::ArrayVec;
use tracing::{debug, error, info};

use super::Machine;

fn parse_state(s: &str) -> u16 {
    let mut state = 0u16;

    let chars = s[1..s.len() - 1].chars();

    for (idx, c) in chars.enumerate() {
        if c == '.' {
            state |= 0 << idx;
        } else if c == '#' {
            state |= 1 << idx;
        } else {
            error!(?c);
            panic!("unexpected character!")
        }
    }

    if state == 0 {
        panic!("unexpected state");
    }

    state
}

fn parse_joltage(s: &str) -> (usize, [u16; 10]) {
    info!(s);

    let (_, s) = s.split_once("{").unwrap();

    let s = s.replace('}', "");

    let nums: Vec<&str> = s.split(',').collect();

    debug!(?nums);

    let num_len = nums.len();

    let mut result = [0u16; 10];

    for (i, num) in nums.into_iter().enumerate() {
        let n: u16 = num.parse().expect("expected str -> u16 conversion");

        result[i] = n;
    }

    (num_len, result)
}

fn buttons_as_vec(buttons: Vec<Vec<u16>>, size: usize) -> ArrayVec<[u16; 10], 10> {
    let mut result: ArrayVec<[u16; 10], 10> = ArrayVec::new();

    debug!(?buttons, size);

    for button in buttons {
        let mut values = [0u16; 10];

        for item in button {
            values[item as usize] = 1;
        }

        result.push(values);
    }

    result
}

fn buttons_as_int(buttons: Vec<Vec<u16>>) -> Vec<u16> {
    let mut result: Vec<u16> = Vec::new();

    for button in buttons.into_iter() {
        let mut value = 0;

        for item in button.into_iter() {
            value |= 1 << item;
        }

        result.push(value);
    }

    result
}

fn parse_buttons(s: &str) -> Vec<Vec<u16>> {
    debug!(s);

    let buttons: Vec<Vec<u16>> = s
        .split("(")
        .filter_map(|s| {
            if let Some((first, _)) = s.split_once(")") {
                return Some(first);
            }

            None
        })
        .map(|s| {
            let s: Vec<&str> = s.split(",").collect();

            s.into_iter().flat_map(|i| i.parse()).collect()
        })
        .collect();

    buttons
}

pub fn parse_str(s: &str) -> Vec<Machine> {
    s.trim()
        .lines()
        .map(|line| {
            let first_split = line.find("] ").expect("expected ] ");

            let (first, rest) = line.split_at(first_split + 1);

            let second_split = rest.find(" {").expect("expected {");

            let (mid, end) = rest.split_at(second_split);

            (first.trim(), mid.trim(), end.trim(), line)
        })
        .map(|i| {
            let (nums_len, joltage) = parse_joltage(i.2);

            let buttons = parse_buttons(i.1);

            let buttons = buttons_as_vec(buttons, joltage.len());

            Machine {
                size: nums_len,
                joltage,
                buttons,
                s: i.3.to_owned(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {

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
    fn parse_test() {
        init_tracing();

        let s = "
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        ";

        let machines = parse_str(s);

        dbg!(machines);

        // let result = main(machines);

        // assert_eq!(33, result);
    }
}
