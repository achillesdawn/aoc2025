use crate::{
    grid::{Direction, Grid},
    seven::recursive::RecurseState,
};

pub fn recurse_up_aided(
    grid: &mut Grid,
    state: &mut RecurseState,
    x: usize,
    y: usize,
    from_dot: bool,
) {
    let Some((c, (new_x, new_y))) = grid.get_direction_with_coords(x, y, &Direction::Up) else {
        return;
    };

    if c == '.' {
        if from_dot {
            return;
        }

        recurse_up_aided(grid, state, new_x + 1, y, true);

        if let Some(subbed) = new_x.checked_sub(1) {
            recurse_up_aided(grid, state, subbed, y, true);
        }
    } else if c == '^' {
        recurse_up_aided(grid, state, new_x, new_y, false);
    } else if c == '|' {
        // grid.set_unchecked(new_x, new_y, 'x');
        recurse_up_aided(grid, state, new_x, new_y, false);
    } else if c == 'x' {
    } else if c == 'S' {
        state.total += 1;
    } else {
        println!("GOT: {}", c);
        panic!("unexpected character");
    }
}
