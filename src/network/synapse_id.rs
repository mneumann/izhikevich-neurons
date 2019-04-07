#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SynapseId(u32);

impl SynapseId {
    #[inline(always)]
    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for SynapseId {
    #[inline(always)]
    fn from(index: usize) -> Self {
        SynapseId(index as u32)
    }
}
