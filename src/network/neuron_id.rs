#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct NeuronId(u32);

impl NeuronId {
    #[inline(always)]
    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for NeuronId {
    #[inline(always)]
    fn from(index: usize) -> Self {
        NeuronId(index as u32)
    }
}
