use crate::grid::Grid;

pub fn process(s: &str) {
    let mut grid = Grid::new(s);

    let mut total = 0usize;

    loop {
        let mut round_total = 0usize;

        for y in 0..grid.cols {
            for x in 0..grid.rows {
                let c = grid.get(x, y).expect("should be in bounds");

                if c != '@' {
                    continue;
                }

                let around = grid.get_all_directions(x, y);

                let sum: usize = around.iter().filter(|c| **c == '@').count();

                if sum < 4 {
                    round_total += 1;

                    grid.set(x, y, '.');
                }
            }
        }

        dbg!(round_total);

        if round_total == 0 {
            break;
        } else {
            total += round_total;
        }
    }

    dbg!(total);
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_one() {
        let s = "..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.";

        process(s);
    }

    #[test]
    fn test_input() {
        let s = read_to_string("src/four/input.txt").expect("could not read input file");

        process(&s);
    }
}
