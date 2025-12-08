use crate::grid::{Direction, Grid};

fn recurse(grid: &mut Grid, x: usize, y: usize) {
    let Some((c, (new_x, new_y))) = grid.get_direction_with_coords(x, y, &Direction::Down) else {
        return;
    };

    if c == '.' {
        grid.set_unchecked(new_x, new_y, '|');

        grid.print_grid();

        recurse(grid, new_x, new_y);
    } else if c == '^' {
        recurse(grid, new_x + 1, y);
        recurse(grid, new_x - 1, y);
    }
}

pub fn parse_input(s: &str) {
    let mut grid = Grid::new(s);

    let Some((x, y)) = grid.find('S') else {
        panic!("could not find start location");
    };

    recurse(&mut grid, x, y);
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
    fn test_input() {
        let s = read_to_string("src/seven/input.txt").expect("expected src/seven/input.txt");

        parse_input(&s);
    }
}
