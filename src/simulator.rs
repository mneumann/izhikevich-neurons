use super::{Num, Timestep, Delay, SynapseId, NeuronId, Network};

// XXX: Make configurable
const STDP_FIRE_RESET: Num = 0.1;
const STDP_DECAY: Num = 0.95;

pub struct Simulator {
    current_time_step: Timestep,

    // We use a cyclic buffer with (time_step % max_delay) as index into the futures_spike array
    future_spikes: Vec<Vec<SynapseId>>,
}

// NOTE: This must be a power of two
const MAX_DELAY: usize = 64;

#[inline(always)]
fn timeslot(at: Timestep) -> usize {
    (at & (MAX_DELAY - 1)) as usize
}

#[inline(always)]
fn timeslot_in_future(at: Timestep, delay: Delay) -> usize {
    timeslot(at + delay as usize)
}

impl Simulator {
    pub fn new(max_delay: usize) -> Simulator {
        assert!(max_delay < MAX_DELAY);
        Simulator {
            current_time_step: 0,
            future_spikes: (0..MAX_DELAY).map(|_| Vec::new()).collect(),
        }
    }

    pub fn current_time_step(&self) -> Timestep {
        self.current_time_step
    }

    /// External input currents have to be set manually by calling `set_external_input`. 

    pub fn step<F>(&mut self, network: &mut Network, fired_callback: &mut F)
        where F: FnMut(NeuronId, Timestep)
    {
        let time_step = self.current_time_step;

        // Clear all input currents
        network.reset_all_input_currents();

        // get all synapse input
        {
            let spikes = &mut self.future_spikes[timeslot(time_step)];
            network.process_firing_synapses(spikes);
            spikes.clear();
        }

        network.update_state(STDP_FIRE_RESET,
                             STDP_DECAY,
                             &mut |syn_id, delay| {
                                 self.future_spikes[timeslot_in_future(time_step, delay)]
                                     .push(syn_id);
                             },
                             &mut |neuron_id| {
                                 fired_callback(neuron_id, time_step);
                             });

        self.current_time_step += 1;
    }
}
