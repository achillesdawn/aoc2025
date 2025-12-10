use crate::grid::{Direction, Grid};

#[derive(Debug, Default)]
struct RecurseState {
    total: usize,
    visited: usize,
}

fn recurse(grid: &mut Grid, state: &mut RecurseState, x: usize, y: usize) {
    let Some((c, (new_x, new_y))) = grid.get_direction_with_coords(x, y, &Direction::Down) else {
        state.total += 1;
        return;
    };

    if c == '.' {
        grid.set_unchecked(new_x, new_y, '|');

        grid.print_grid();

        recurse(grid, state, new_x, new_y);
    } else if c == '^' {
        recurse(grid, state, new_x + 1, y);
        recurse(grid, state, new_x - 1, y);
    } else if c == '|' {
        state.visited += 1;
    } else {
        panic!("unexpected character");
    }
}

pub fn parse_input(s: &str) {
    let mut grid = Grid::new(s);

    let Some((x, y)) = grid.find('S') else {
        panic!("could not find start location");
    };

    let mut state = RecurseState::default();

    recurse(&mut grid, &mut state, x, y);

    grid.print_grid();

    dbg!(state);
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_one() {
        let s = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        parse_input(s);
    }

    #[test]
    fn test_two() {
        let s = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............";

        parse_input(s);
    }

    #[test]
    fn test_input() {
        let s = read_to_string("src/seven/input.txt").expect("expected src/seven/input.txt");

        parse_input(&s);
    }
}
