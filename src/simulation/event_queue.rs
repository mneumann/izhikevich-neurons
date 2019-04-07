use std::collections::BinaryHeap;
use crate::simulation::{Timestep, Event};

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
