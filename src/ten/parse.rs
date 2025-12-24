use super::Machine;
use tracing::{error, info};

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

    state
}

fn parse_buttons(s: &str) -> Vec<u16> {
    info!(s);

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

    info!(?buttons);

    let mut masks: Vec<u16> = Vec::new();

    for button in buttons.into_iter() {
        let mut mask = 0u16;

        for item in button.into_iter() {
            mask |= 1 << item;
        }

        masks.push(mask);
    }

    masks
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
            let state = parse_state(i.0);

            let masks = parse_buttons(i.1);

            Machine {
                state,
                masks,
                s: i.3.to_owned(),
                width: i.0.len() - 2,
            }
        })
        .collect()
}
