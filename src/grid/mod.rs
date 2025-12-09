mod direction;
mod find;
use std::collections::HashSet;

pub use direction::Direction;

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<char>>,

    pub paths: HashSet<Vec<u8>>,

    pub cols: usize,
    pub rows: usize,
}

impl Grid {
    pub fn new(s: &str) -> Self {
        let grid: Vec<Vec<char>> = s
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();

        let cols = grid.len();
        let rows = grid[0].len();

        Self {
            grid,
            cols,
            rows,
            paths: HashSet::new(),
        }
    }

    pub fn get_checked(&self, x: usize, y: usize) -> Option<char> {
        if (0..self.rows).contains(&x) && (0..self.cols).contains(&y) {
            Some(self.grid[y][x])
        } else {
            None
        }
    }

    fn get_checked_with_coords(&self, x: usize, y: usize) -> Option<(char, (usize, usize))> {
        let c = self.get_checked(x, y)?;

        Some((c, (x, y)))
    }

    fn get_unchecked(&self, x: usize, y: usize) -> char {
        self.grid[y][x]
    }

    fn get_unchecked_with_coords(&self, x: usize, y: usize) -> (char, (usize, usize)) {
        (self.get_unchecked(x, y), (x, y))
    }

    pub fn set_unchecked(&mut self, x: usize, y: usize, c: char) {
        self.grid[y][x] = c;
    }

    fn x_plus(&self, x: usize) -> Option<usize> {
        if x >= self.rows {
            return None;
        }

        Some(x + 1)
    }

    fn y_plus(&self, y: usize) -> Option<usize> {
        if y >= self.cols {
            return None;
        }

        Some(y + 1)
    }

    fn x_or_y_minus(&self, x_or_y: usize) -> Option<usize> {
        x_or_y.checked_sub(1)
    }

    pub fn direction_coords(
        &self,
        x: usize,
        y: usize,
        direction: &Direction,
    ) -> Option<(usize, usize)> {
        match direction {
            Direction::Up => {
                let y = self.x_or_y_minus(y)?;
                Some((x, y))
            }
            Direction::Down => {
                let y = self.y_plus(y)?;
                Some((x, y))
            }
            Direction::Left => {
                let x = self.x_or_y_minus(x)?;
                Some((x, y))
            }
            Direction::Right => {
                let x = self.x_plus(x)?;

                Some((x, y))
            }
            Direction::UpRight => {
                let y = self.x_or_y_minus(y)?;
                let x = self.x_plus(x)?;

                Some((x, y))
            }
            Direction::UpLeft => {
                let y = self.x_or_y_minus(y)?;
                let x = self.x_or_y_minus(x)?;

                Some((x, y))
            }
            Direction::DownRight => {
                let y = self.y_plus(y)?;
                let x = self.x_plus(x)?;

                Some((x, y))
            }
            Direction::DownLeft => {
                let y = self.y_plus(y)?;

                let x = self.x_or_y_minus(x)?;

                Some((x, y))
            }
        }
    }

    pub fn get_direction(&self, x: usize, y: usize, direction: &Direction) -> Option<char> {
        let (x, y) = self.direction_coords(x, y, direction)?;

        Some(self.get_unchecked(x, y))
    }

    pub fn get_direction_with_coords(
        &self,
        x: usize,
        y: usize,
        direction: &Direction,
    ) -> Option<(char, (usize, usize))> {
        let (x, y) = self.direction_coords(x, y, direction)?;

        self.get_checked_with_coords(x, y)
    }

    pub fn get_all_directions(&self, x: usize, y: usize) -> Vec<char> {
        let mut results = Vec::new();

        let directions = Direction::items();

        for direction in directions {
            if let Some(c) = self.get_direction(x, y, &direction) {
                results.push(c);
            }
        }

        results
    }

    pub fn print_grid(&self) {
        for y in 0..self.cols {
            for x in 0..self.rows {
                let c = self.get_unchecked(x, y);
                print!("{}", c);
            }

            println!();
        }

        println!()
    }
}
