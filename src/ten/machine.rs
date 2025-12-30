use tracing::debug;

#[derive(Debug)]
pub struct Machine {
    pub buttons: Vec<Vec<u16>>,
    pub joltage: Vec<u16>,
    pub s: String,
}

impl Machine {
    pub fn calculate_state(&self, state: &[usize]) -> Vec<u16> {
        let mut sum: Vec<u16> = vec![0u16; self.joltage.len()];

        for button_idx in state.iter() {
            let button = self
                .buttons
                .get(*button_idx)
                .expect("expected state to be in range");

            debug!(?button);

            sum.iter_mut().zip(button).for_each(|(s, b)| *s += *b);
        }

        sum
    }
}
