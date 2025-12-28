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

fn parse_joltage(s: &str) -> Vec<u16> {
    info!(s);

    let (_, s) = s.split_once("{").unwrap();

    let s = s.replace('}', "");

    let nums: Vec<&str> = s.split(',').collect();

    debug!(?nums);

    nums.into_iter().flat_map(|i| i.parse()).collect()
}

fn parse_buttons(s: &str, size: usize) -> Vec<Vec<u16>> {
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

    let mut result: Vec<Vec<u16>> = Vec::new();

    debug!(?buttons, size);

    for button in buttons {
        let mut values: Vec<u16> = vec![0u16; size];

        for item in button {
            values[item as usize] = 1;
        }

        result.push(values);
    }

    result
}

pub fn parse_str(s: &str) -> Vec<Machine> {
    s.trim()
        .lines()
        .map(|l| {
            let first_split = l.find("] ").expect("expected ] ");

            let (first, rest) = l.split_at(first_split + 1);

            let second_split = rest.find(" {").expect("expected {");

            let (mid, end) = rest.split_at(second_split);

            (first.trim(), mid.trim(), end.trim(), l)
        })
        .map(|i| {
            let joltage = parse_joltage(i.2);

            let masks = parse_buttons(i.1, joltage.len());

            Machine {
                width: joltage.len(),
                joltage,
                masks,
                s: i.3.to_owned(),
            }
        })
        .collect()
}
