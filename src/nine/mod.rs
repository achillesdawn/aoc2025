use crate::grid::Grid;

pub fn parse(s: &str) -> Vec<(usize, usize)> {
    s.lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(first, second)| (first.trim(), second.trim()))
        .map(|(first, second)| (first.parse().unwrap(), second.parse().unwrap()))
        .collect()
}

fn create_grid(positions: Vec<(usize, usize)>) {
    let grid = Grid::new(10, 2);

    grid.print_grid();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_one() {
        let s = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        let positions = parse(s);

        create_grid(positions);
    }
}
