use crate::Num;
use std::cmp::Ordering;
use crate::network::NeuronId;
use crate::simulation::Timestep;

#[derive(Debug, Copy, Clone)]
pub struct Event {
    pub at: Timestep,
    pub neuron: NeuronId,
    pub weight: Num,
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.at == other.at && self.neuron == other.neuron
    }
}
impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.at.partial_cmp(&other.at).map(|o| o.reverse())
    }
}

impl Ord for Event {
    // The `cmp` function is required for the sort order in the
    // BinaryHeap. As `std::collections::BinaryHeap` implements
    // a max-heap, but we require a min-heap, we have to use the
    // `reverse` ordering here.
    fn cmp(&self, other: &Self) -> Ordering {
        self.at.cmp(&other.at).reverse()
    }
}
