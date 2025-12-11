use std::collections::HashMap;

use crate::grid::{Direction, Grid};

#[derive(Debug, Default)]
pub struct RecurseState {
    pub total: usize,
    pub max_depth: usize,
    pub ends: HashMap<(usize, usize), usize>,
}

impl RecurseState {
    pub fn new(max_depth: usize) -> Self {
        Self {
            total: 0,
            max_depth,
            ends: HashMap::new(),
        }
    }
}

pub fn recurse_to_depth(
    grid: &mut Grid,
    state: &mut RecurseState,
    x: usize,
    y: usize,
    depth: usize,
) {
    let Some((c, (new_x, new_y))) = grid.get_direction_with_coords(x, y, &Direction::Down) else {
        // state.total += 1;
        return;
    };

    if c == '.' || c == '|' {
        grid.set_unchecked(new_x, new_y, '|');

        if depth >= state.max_depth {
            state.total += 1;

            let entry = state.ends.entry((new_x, new_y)).or_insert(0);
            *entry += 1;

            return;
        }

        recurse_to_depth(grid, state, new_x, new_y, depth);
    } else if c == '^' {
        recurse_to_depth(grid, state, new_x + 1, y, depth + 1);
        recurse_to_depth(grid, state, new_x - 1, y, depth + 1);
    // } else  c == '|' {
    } else {
        panic!("unexpected character");
    }
}
