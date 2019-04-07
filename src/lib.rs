/// We use this numerical type for all neuron model parameters (e.g. potential).
pub type Num = f32;

pub struct Closed01(Num);

impl Closed01 {
    pub fn new(f: Num) -> Self {
        assert!(f >= 0.0 && f <= 1.0);
        Self(f)
    }
    pub fn get(self) -> Num {
        self.0
    }
}

pub mod model;
pub mod network;
pub mod simulation;
