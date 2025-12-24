use std::fmt::Display;

#[derive(Debug)]
pub struct Machine {
    pub state: u16,
    pub masks: Vec<u16>,
    pub s: String,
    pub width: usize,
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for mask in self.masks.iter() {
            s.push_str(&format!(" {mask:0width$b} ", width = self.width));
        }

        write!(
            f,
            "{}\n[{:0width$b}]  {}\n[{}] {:?}",
            self.s.trim(),
            self.state,
            s,
            self.state,
            self.masks,
            width = self.width,
        )
    }
}
