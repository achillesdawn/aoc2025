use std::collections::BTreeMap;

use crate::{
    grid::Grid,
    seven::recursive::{recurse, recurse_to_depth},
};

mod recursive;
use recursive::RecurseState;

pub fn parse_input(s: &str) {
    let mut grid = Grid::new(s);

    let Some((x, y)) = grid.find('S') else {
        panic!("could not find start location");
    };

    let mut state = RecurseState::new(70);

    recurse_to_depth(&mut grid, &mut state, x, y, 0);
    // recurse(&mut grid, &mut state, x, y);

    grid.print_grid();

    dbg!(state);
}

pub fn start_from(s: &str) {
    let mut grid = Grid::new(s);

    let ends: BTreeMap<(usize, usize), usize> = BTreeMap::from([
        ((35, 71), 1),
        ((37, 71), 3),
        ((39, 71), 11),
        ((41, 71), 9),
        ((42, 71), 22),
        ((43, 71), 79),
        ((45, 71), 325),
        ((47, 71), 246),
        ((48, 71), 888),
        ((49, 71), 2027),
        ((51, 71), 5201),
        ((53, 71), 3174),
        ((54, 71), 1500),
        ((55, 71), 84334),
        ((57, 71), 62852),
        ((58, 71), 112218),
        ((60, 71), 102231),
        ((61, 71), 541568),
        ((63, 71), 270784),
        ((64, 71), 425595),
        ((65, 71), 2789435),
        ((67, 71), 5367323),
        ((69, 71), 3831687),
        ((70, 71), 3888322),
        ((71, 71), 2513578),
        ((73, 71), 3846076),
        ((75, 71), 1370897),
        ((77, 71), 173019),
        ((78, 71), 72268),
        ((80, 71), 135084),
        ((82, 71), 62816),
        ((83, 71), 21368),
        ((85, 71), 17677),
        ((87, 71), 2710),
        ((88, 71), 4691),
        ((89, 71), 2136),
        ((91, 71), 2136),
        ((92, 71), 352),
        ((94, 71), 74),
        ((95, 71), 58),
        ((97, 71), 89),
        ((99, 71), 41),
        ((101, 71), 12),
        ((103, 71), 3),
        ((105, 71), 1),
    ]);

    let mut state = RecurseState {
        total: 25714921,
        max_depth: 70,
        ends: BTreeMap::new(),
    };

    for ((x, y), value) in ends.into_iter() {
        recurse(&mut grid, &mut state, x, y);

        grid.print_grid();

        break;
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
    fn test_two() {
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
...............";

        parse_input(s);
    }

    #[test]
    fn test_input() {
        let s = read_to_string("src/seven/input.txt").expect("expected src/seven/input.txt");

        parse_input(&s);
    }

    #[test]
    fn intwo() {
        let s = read_to_string("src/seven/input.txt").expect("expected src/seven/input.txt");

        start_from(&s);
    }
}
