#[derive(Debug)]
pub struct Machine {
    pub buttons: Vec<[u16; 10]>,
    pub joltage: [u16; 10],
    pub s: String,
    pub size: usize,
}

impl Machine {
    pub fn calculate_state(&self, state: &[usize], result: &mut [u16]) {
        // clear state
        result.iter_mut().for_each(|i| *i = 0);

        for button_idx in state.iter() {
            let button = self
                .buttons
                .get(*button_idx)
                .expect("expected state to be in range");

            result[..self.size]
                .iter_mut()
                .zip(button)
                .for_each(|(s, b)| *s += *b);
        }
    }
}
