#[derive(Debug)]
pub struct Machine {
    pub masks: Vec<Vec<u16>>,
    pub joltage: Vec<u16>,
    pub s: String,
    pub width: usize,
}
