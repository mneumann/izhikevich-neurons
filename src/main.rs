extern crate izhikevich_neurons;
extern crate gnuplot;

use izhikevich_neurons::{Config, State, Num};
use gnuplot::{Figure, Caption, Color, AxesCommon};

type NeuronId = u32;
type SynapseId = u32;
type TimeStep = u32;
type Delay = u8;

const MAX_DELAY: u8 = 40;

struct Synapse {
    pre_neuron: NeuronId,
    post_neuron: NeuronId,
    delay: Delay,
    weight: Num,
    last_spike: TimeStep,
    // ... learning parameters
}

struct Neuron {
    state: State,
    config: Config,
    i_ext: Num,
    i_inp: Num,
    pre_synapses: Vec<SynapseId>,
    post_synapses: Vec<SynapseId>,
}

struct Network {
    neurons: Vec<Neuron>,
    synapses: Vec<Synapse>,
}

impl Network {
    fn new() -> Network {
        Network {
            neurons: Vec::new(),
            synapses: Vec::new()
        }
    }

    fn create_neuron(&mut self, config: Config) -> NeuronId {
        let neuron = Neuron {
            state: State::new(),
            config: config,
            i_ext: 0.0,
            i_inp: 0.0,
            pre_synapses: Vec::new(),
            post_synapses: Vec::new(),
        };
        let neuron_id = self.neurons.len() as u32;
        self.neurons.push(neuron);
        return neuron_id;
    }

    fn connect(&mut self, pre_neuron: NeuronId, post_neuron: NeuronId, delay: Delay, weight: Num) -> SynapseId {
        assert!((pre_neuron as usize) < self.neurons.len());
        assert!((post_neuron as usize) < self.neurons.len());
        assert!(delay > 0);
        assert!(delay <= MAX_DELAY);

        let synapse = Synapse {
            pre_neuron: pre_neuron,
            post_neuron: post_neuron,
            delay: delay, 
            weight: weight,
            last_spike: 0, // XXX
        };
        let synapse_id = self.synapses.len() as u32;

        self.synapses.push(synapse);
        self.neurons[pre_neuron as usize].post_synapses.push(synapse_id);
        self.neurons[post_neuron as usize].pre_synapses.push(synapse_id);

        return synapse_id;
    }
}

fn main() {
    let config = Config::regular_spiking();

    const PARAMS : &'static [(&'static str, Num, &'static str)] = &[
        ("Neuron 1 [7 pA current 200..700ms]", 7.0, "blue"),
        //("Neuron 2 [2.69 pA current 200..700ms]", 2.69, "red"),
        ("Neuron 2 [0.0 pA current 200..700ms]", 0.0, "red"),
        //("Neuron 3 [2.7 pA current 200..700ms]", 2.7, "green")
    ];

    let mut network = Network::new();

    let n1 = network.create_neuron(Config::regular_spiking());
    let n2 = network.create_neuron(Config::regular_spiking());
    //let n3 = network.create_neuron(Config::regular_spiking());

    // We use a cyclic buffer
    // We use (time_step % MAX_DELAY) as index into the futures_spike array
    let mut future_spikes: Vec<Vec<SynapseId>> = (0..MAX_DELAY as usize).map(|_| Vec::new()).collect();

    let _ = network.connect(n1, n2, 10, 7.0);
    let _ = network.connect(n1, n2, 5, 7.0);
    let _ = network.connect(n1, n2, 2, 7.0);
    let _ = network.connect(n1, n2, 2, 7.0);
    let _ = network.connect(n2, n2, 40, 17.0);
    let _ = network.connect(n2, n2, 40, 7.0);

    let mut states: Vec<_> = PARAMS.iter().map(|_| Vec::new()).collect();

    let mut time_step: TimeStep = 0;

    while time_step <= 1_000 {
        // record current state
        for (i, neuron) in network.neurons.iter().enumerate() {
            states[i].push(neuron.state);
        }
        time_step += 1;

        // Clear all input currents
        for neuron in network.neurons.iter_mut() {
            neuron.i_inp = 0.0;
        }

        // get all synapse input
        {
            let current_spikes = &mut future_spikes[(time_step % (MAX_DELAY as TimeStep)) as usize];
            for &syn_fired in current_spikes.iter() {
                println!("time: {}. input from synapse: {}", time_step, syn_fired); 
                let (weight, post_neuron) = {
                    let syn = &network.synapses[syn_fired as usize];
                    (syn.weight, syn.post_neuron)
                };
                network.neurons[post_neuron as usize].i_inp += weight; 
            }
            current_spikes.clear();
        }

        // update state
        for (i, mut neuron) in network.neurons.iter_mut().enumerate() {
            if time_step >= 200 && time_step <= 700 {
                neuron.i_ext = PARAMS[i].1;
            } else {
                neuron.i_ext = 0.0;
            }

            let syn_i = neuron.i_ext + neuron.i_inp;

            let (new_state, fired) = neuron.state.step_1ms(syn_i, &neuron.config);
            neuron.state = new_state;
            if fired {
                println!("Neuron {} fired at {} ms", i, time_step);
                for &syn_id in neuron.post_synapses.iter() {
                    let future = time_step + network.synapses[syn_id as usize].delay as TimeStep;
                    future_spikes[future as usize % MAX_DELAY as usize].push(syn_id);
                }
            }
        }
    }

    {
        let mut fg = Figure::new();
        {
            let mut diag = fg.axes2d().
                set_x_label("time (ms)", &[]).
                set_y_label("membrane potential v (mV)", &[]);
            for (i, &p) in PARAMS.iter().enumerate() {
                diag.lines(states[i].iter().enumerate().map(|(i, _)| i as f32), states[i].iter().map(|s| s.potential()), &[Caption(p.0), Color(p.2)]);
            }
        }
        fg.show();
    }


    /*
    let mut fg = Figure::new();
    {
        let mut diag = fg.axes2d().
            set_x_label("membrane potential v (mV)", &[]).
            set_y_label("recovery variable u", &[]);
        for (i, &p) in PARAMS.iter().enumerate() {
            diag.lines(states[i].iter().map(|s| s.potential()), states[i].iter().map(|s| s.recovery()), &[Caption(p.0), Color(p.2)]);
        }
    }
    fg.show();
    */

}
