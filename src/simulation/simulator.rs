use crate::model::StdpConfig;
use crate::network::{Network, NeuronId, SynapseDelay, SynapseId};
use crate::simulation::Timestep;

pub struct Simulator {
    current_time_step: Timestep,

    // We use a cyclic buffer with (time_step % max_delay) as index into the futures_spike array
    future_spikes: Vec<Vec<SynapseId>>,

    /// The next power of two of `max_delay` - 1. This is used as
    /// bit-wise AND mask, to avoid using the expensive modulo (%)
    /// operator.
    max_delay_bitwise_and_mask: usize,

    /// Spike Time Dependent Plasticity configuration
    stdp_config: StdpConfig,
}

impl Simulator {
    /// Creates a new Simulator.
    ///
    /// `max_delay`: The maximum delay a synapse can have. We use this value to size
    /// our `future_spikes` array.
    ///
    /// `stdp_config`: STDP configuration
    ///
    pub fn new(max_delay: SynapseDelay, stdp_config: StdpConfig) -> Simulator {
        let max_delay = max_delay.get() as usize;
        assert!(max_delay > 1);
        let next_power_of_two = max_delay.checked_next_power_of_two().unwrap();
        assert!(next_power_of_two >= max_delay);
        let max_delay_bitwise_and_mask = next_power_of_two - 1;

        Simulator {
            current_time_step: 0,
            future_spikes: (0..next_power_of_two).map(|_| Vec::new()).collect(),
            max_delay_bitwise_and_mask,
            stdp_config,
        }
    }

    #[inline(always)]
    fn timeslot(&self, at: Timestep) -> usize {
        (at as usize) & self.max_delay_bitwise_and_mask
    }

    #[inline(always)]
    fn timeslot_in_future(&self, at: Timestep, synapse_delay: SynapseDelay) -> usize {
        self.timeslot(at + synapse_delay.get() as usize)
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

        for i in 0..network.neurons.len() {
            let activity = network.neurons[i].update_state(self.stdp_config);

            if activity.fires() {
                let neuron_id = NeuronId::from(i);
                fired_callback(neuron_id, time_step);

                for &syn_id in network.neurons[i].post_synapses.iter() {
                    let synapse_delay = network.synapses[syn_id.index()].synapse_delay;
                    let idx = self.timeslot_in_future(time_step, synapse_delay);
                    self.future_spikes[idx].push(syn_id);
                }

                network.excite_all_pre_synapses_of_neuron(neuron_id);
            }
        }

        self.current_time_step += 1;
    }
}
