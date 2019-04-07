pub mod event_queue;
pub mod fire_recorder;
pub mod simulator;

pub type Timestep = usize;

pub use fire_recorder::FireRecorder;
pub use simulator::Simulator;
