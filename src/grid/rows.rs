use super::Grid;

impl Grid {
    pub fn get_row(&self, col: usize) -> Vec<char> {
        self.grid[col].clone()
    }
}
