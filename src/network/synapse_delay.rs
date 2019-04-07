#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
pub struct SynapseDelay(u8);

impl SynapseDelay {
    pub fn new(delay: u8) -> Self {
        assert!(delay > 0);
        Self(delay)
    }

    pub fn get(self) -> u8 {
        self.0
    }
}
