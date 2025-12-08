use crate::grid::{Direction, Grid};

pub fn parse_input(s: &str) {
    let grid = Grid::new(s);

    let Some((mut x, mut y)) = grid.find('S') else {
        panic!("could not find start location");
    };

    loop {
        let Some((c, (new_x, new_y))) = grid.get_direction_with_coords(x, y, &Direction::Down)
        else {
            panic!("end");
        };

        x = new_x;
        y = new_y;

        dbg!(x, y, c);
    }
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
    }
}
