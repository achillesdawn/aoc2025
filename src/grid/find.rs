use super::Grid;

impl Grid {
    pub fn find(&self, needle: char) -> Option<(usize, usize)> {
        for y in 0..self.cols {
            for x in 0..self.rows {
                let c = self.get_unchecked(x, y);

                if c == needle {
                    return Some((x, y));
                }
            }
        }

        None
    }
}
