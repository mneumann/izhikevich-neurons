use Num;

/// Spike-Time Dependent Plasticity (STDP) configuration
#[derive(Debug, Copy, Clone)]
pub struct StdpConfig {
    /// By how much the `stdp` value of each neuron decays during every
    /// simulator time step.
    pub decay: Num,

    /// The `stdp` value is reset to `fire_reset` when the neuron fires.
    pub fire_reset: Num,
}

impl Default for StdpConfig {
    fn default() -> Self {
        Self {
            decay: 0.95,
            fire_reset: 0.1,
        }
    }
}
