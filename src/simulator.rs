use {Delay, Network, NeuronId, Num, SynapseId, Timestep};

// XXX: Make configurable
const STDP_FIRE_RESET: Num = 0.1;
const STDP_DECAY: Num = 0.95;

pub struct Simulator {
    current_time_step: Timestep,

    // We use a cyclic buffer with (time_step % max_delay) as index into the futures_spike array
    future_spikes: Vec<Vec<SynapseId>>,

    /// The next power of two of `max_delay` - 1. This is used as
    /// bit-wise AND mask, to avoid using the expensive modulo (%)
    /// operator.
    max_delay_bitwise_and_mask: usize,
}

impl Simulator {
    pub fn new(max_delay: usize) -> Simulator {
        assert!(max_delay > 1);
        let next_power_of_two = max_delay.checked_next_power_of_two().unwrap();
        assert!(next_power_of_two >= max_delay);
        let max_delay_bitwise_and_mask = next_power_of_two - 1;

        Simulator {
            current_time_step: 0,
            future_spikes: (0..next_power_of_two).map(|_| Vec::new()).collect(),
            max_delay_bitwise_and_mask,
        }
    }

    #[inline(always)]
    fn timeslot(&self, at: Timestep) -> usize {
        (at as usize) & self.max_delay_bitwise_and_mask
    }

    #[inline(always)]
    fn timeslot_in_future(&self, at: Timestep, delay: Delay) -> usize {
        self.timeslot(at + delay as usize)
    }

    pub fn current_time_step(&self) -> Timestep {
        self.current_time_step
    }

    /// External input currents have to be set manually by calling `set_external_input`.

    pub fn step<F>(&mut self, network: &mut Network, fired_callback: &mut F)
    where
        F: FnMut(NeuronId, Timestep),
    {
        let time_step = self.current_time_step;

        // Clear all input currents
        network.reset_all_input_currents();

        // get all synapse input
        {
            let idx = self.timeslot(time_step);
            let spikes = &mut self.future_spikes[idx];
            network.process_firing_synapses(spikes);
            spikes.clear();
        }

        network.update_state(
            STDP_FIRE_RESET,
            STDP_DECAY,
            &mut |syn_id, delay| {
                let idx = self.timeslot_in_future(time_step, delay);
                self.future_spikes[idx].push(syn_id);
            },
            &mut |neuron_id| {
                fired_callback(neuron_id, time_step);
            },
        );

        self.current_time_step += 1;
    }
}
