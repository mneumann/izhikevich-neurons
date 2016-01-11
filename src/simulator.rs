use super::{Num, TimeStep, SynapseId, NeuronId, Network};

// XXX: Make configurable
const STDP_FIRE_RESET: Num = 0.1;
const STDP_DECAY: Num = 0.95;

pub struct Simulator {
    current_time_step: TimeStep,
    max_delay: usize,

    // We use a cyclic buffer
    // We use (time_step % max_delay) as index into the futures_spike array
    future_spikes: Vec<Vec<SynapseId>>,
}

impl Simulator {
    pub fn new(max_delay: usize) -> Simulator {
        Simulator {
            current_time_step: 0,
            max_delay: max_delay,
            future_spikes: (0..(max_delay + 1)).map(|_| Vec::new()).collect(),
        }
    }

    pub fn current_time_step(&self) -> TimeStep {
        self.current_time_step
    }

    pub fn step<F>(&mut self,
                   network: &mut Network,
                   external_inputs: &[(NeuronId, TimeStep, Num)],
                   fired_callback: &mut F)
        where F: FnMut(NeuronId, TimeStep)
    {
        let time_step = self.current_time_step;

        // Clear all input currents
        network.clear_all_input_currents();

        // get all synapse input
        {
            let time_slot = (time_step % (self.max_delay as TimeStep)) as usize;
            let spikes = &mut self.future_spikes[time_slot];
            network.process_firing_synapses(spikes);
            spikes.clear();
        }

        // set external inputs
        // XXX
        for &(neuron_id, at, current) in external_inputs {
            if time_step == at {
                network.set_external_input(neuron_id, current);
            }
        }

        network.update_state(STDP_FIRE_RESET,
                             STDP_DECAY,
                             &mut |syn_id, delay| {
                                 let future = time_step + delay as TimeStep;
                                 let max_delay = self.future_spikes.len();
                                 self.future_spikes[future as usize % max_delay].push(syn_id);
                             },
                             &mut |neuron_id| {
                                 fired_callback(neuron_id, time_step);
                             });

        self.current_time_step += 1;
    }
}
