use std::collections::BTreeMap;

mod up;

pub use up::recurse_up_aided;

use crate::grid::{Direction, Grid};

#[derive(Debug, Default)]
pub struct RecurseState {
    pub total: usize,
    pub max_depth: usize,
    pub ends: BTreeMap<(usize, usize), usize>,
}

impl RecurseState {
    pub fn new(max_depth: usize) -> Self {
        Self {
            total: 0,
            max_depth,
            ends: BTreeMap::new(),
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
        panic!("should not get here yet");
    };

    if depth >= state.max_depth {
        state.total += 1;

        let entry = state.ends.entry((new_x, new_y)).or_insert(0);
        *entry += 1;

        if c == '.' || c == '|' {
            grid.set_unchecked(new_x, new_y, '|');
        }

        return;
    }

    if c == '.' || c == '|' {
        grid.set_unchecked(new_x, new_y, '|');

        recurse_to_depth(grid, state, new_x, new_y, depth + 1);
    } else if c == '^' {
        grid.set_unchecked(new_x + 1, new_y, '|');
        recurse_to_depth(grid, state, new_x + 1, new_y, depth + 1);

        grid.set_unchecked(new_x - 1, new_y, '|');
        recurse_to_depth(grid, state, new_x - 1, new_y, depth + 1);
    // } else  c == '|' {
    } else {
        panic!("unexpected character");
    }
}

pub fn recurse(grid: &mut Grid, state: &mut RecurseState, x: usize, y: usize) {
    let Some((c, (new_x, new_y))) = grid.get_direction_with_coords(x, y, &Direction::Down) else {
        state.total += 1;
        return;
    };

    if c == '^' {
        grid.set_unchecked(new_x + 1, new_y, '|');
        recurse(grid, state, new_x + 1, new_y);

        grid.set_unchecked(new_x - 1, new_y, '|');
        recurse(grid, state, new_x - 1, new_y);
    } else {
        grid.set_unchecked(new_x, new_y, '|');

        // grid.print_grid();

        recurse(grid, state, new_x, new_y);
    }
}

pub fn recurse_once(grid: &mut Grid, state: &mut RecurseState, x: usize, y: usize) {
    let Some((c, (new_x, new_y))) = grid.get_direction_with_coords(x, y, &Direction::Down) else {
        state.total += 1;
        return;
    };

    if c == '.' {
        grid.set_unchecked(new_x, new_y, '|');

        recurse_once(grid, state, new_x, new_y);
    } else if c == '^' {
        grid.set_unchecked(new_x + 1, new_y, '|');
        recurse_once(grid, state, new_x + 1, new_y);

        grid.set_unchecked(new_x - 1, new_y, '|');
        recurse_once(grid, state, new_x - 1, new_y);
    } else if c == '|' {
    } else {
        panic!("unexpected character");
    }
}
