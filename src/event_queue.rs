use std::cmp::Ordering;
use std::collections::BinaryHeap;
use {NeuronId, Num, Timestep};

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

pub struct EventQueue {
    heap: BinaryHeap<Event>,
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, ev: Event) {
        self.heap.push(ev);
    }

    // Returns the next event that has a timestamp of `at`. Returns
    // None, if the event queue is either empty, or if there is no
    // event for timestamp `at`. Panics if the top-most event in the
    // event queue is before `at`.
    pub fn pop_next_event_at(&mut self, at: Timestep) -> Option<Event> {
        match self.heap.peek() {
            Some(ev) if ev.at <= at => {
                // fall-through, because of the borrow checker
            }
            _ => {
                return None;
            }
        }
        let ev = self.heap.pop().unwrap();
        assert!(ev.at == at, "old event found");
        return Some(ev);
    }
}
