use super::{NeuronId, Num, Timestep};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

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

    pub fn pop_next_event_at(&mut self, at: Timestep) -> Option<Event> {
        match self.heap.peek() {
            Some(ev) if at >= ev.at => {}
            _ => {
                return None;
            }
        }
        let ev = self.heap.pop().unwrap();
        if at > ev.at {
            panic!("old event found");
        }
        debug_assert!(ev.at == at);
        return Some(ev);
    }
}
