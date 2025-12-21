use crate::grid::Grid;

pub fn parse(s: &str) -> Vec<(isize, isize)> {
    s.lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(first, second)| (first.trim(), second.trim()))
        .map(|(first, second)| (first.parse().unwrap(), second.parse().unwrap()))
        .collect()
}

fn calc_area(pos: &(isize, isize), other: &(isize, isize)) -> isize {
    let dim_a = (pos.1 - other.1).abs() + 1;
    let dim_b = (other.0 - pos.0).abs() + 1;

    dim_a * dim_b
}

fn iterate(positions: Vec<(isize, isize)>) -> isize {
    let mut max_area = 0isize;

    let mut max_pos = &(0isize, 0isize);
    let mut max_other = &(0isize, 0isize);

    for pos in positions.iter() {
        for other in positions.iter() {
            let area = calc_area(pos, other);

            if area > max_area {
                max_area = area;
                max_pos = pos;
                max_other = other;
            }
        }
    }

    dbg!(max_area, max_other, max_pos);

    max_area
}

// fn positions_min_max(positions: &[(isize, isize)]) -> (isize, isize, isize, isize) {
//     let mut x_min = isize::MAX;
//     let mut x_max = 0isize;
//     let mut y_min = isize::MAX;
//     let mut y_max = 0isize;

//     for (x, y) in positions.iter() {
//         if *x < x_min {
//             x_min = *x;
//         }
//         if *x > x_max {
//             x_max = *x;
//         }

//         if *y < y_min {
//             y_min = *y;
//         }
//         if *y > y_max {
//             y_max = *y;
//         }
//     }

//     (x_min, x_max, y_min, y_max)
// }

fn interpolate(grid: &mut Grid, position: (isize, isize), last: (isize, isize)) {
    let delta_x = position.0 - last.0;
    let delta_y = position.1 - last.1;

    if delta_y == 0 && delta_x == 0 {
        panic!("expected delta x or delta y to be zero");
    } else if delta_x != 0 {
        println!("setting direction");

        let direction = delta_x > 0;

        for i in 1..delta_x.abs() {
            if direction {
                grid.set_unchecked((last.0 + i) as usize, last.1 as usize, '+');
            } else {
                grid.set_unchecked((last.0 - i) as usize, last.1 as usize, '+');
            }
        }
    } else if delta_y != 0 {
        dbg!(delta_y);

        let direction = delta_y > 0;

        for i in 1..delta_y.abs() {
            if direction {
                grid.set_unchecked((last.0) as usize, (last.1 + i) as usize, '+');
            } else {
                grid.set_unchecked((last.0) as usize, (last.1 - i) as usize, '+');
            }
        }
    }
}
fn create_grid(positions: Vec<(isize, isize)>) {
    let mut grid = Grid::new(14, 9);

    let mut last: (isize, isize) = (0, 0);

    for (i, (x, y)) in positions.iter().enumerate() {
        grid.set_unchecked(*x as usize, *y as usize, '#');

        if i == 0 {
            last = (*x, *y);
            continue;
        }

        interpolate(&mut grid, (*x, *y), last);

        grid.print();

        last = (*x, *y);
    }

    grid.print();
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

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

    #[test]
    fn test_area() {
        let area = calc_area(&(2, 5), &(11, 1));

        assert_eq!(area, 50);
    }

    #[test]
    fn with_input() {
        let s = read_to_string("src/nine/input.txt").unwrap();

        let positions = parse(&s);

        iterate(positions);
    }
}
