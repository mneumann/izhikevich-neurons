use crate::model::{NeuronConfig, NeuronState};
use crate::network::{Network, Neuron, NeuronId, Synapse, SynapseDelay, SynapseId};
use crate::Num;

#[derive(Debug)]
pub struct NetworkBuilder {
    neurons: Vec<Neuron>,
    synapses: Vec<Synapse>,
}

impl NetworkBuilder {
    pub fn new() -> NetworkBuilder {
        NetworkBuilder {
            neurons: Vec::new(),
            synapses: Vec::new(),
        }
    }

    pub fn create_neuron(&mut self, config: NeuronConfig) -> NeuronId {
        let neuron = Neuron {
            state: NeuronState::new(),
            config: config,
            stdp: 0.0,
            i_ext: 0.0, // external current
            i_inp: 0.0, // synaptic current
            // connectivity
            pre_synapses: Vec::new(),
            post_synapses: Vec::new(),
        };
        let neuron_id = NeuronId::from(self.neurons.len());
        self.neurons.push(neuron);
        return neuron_id;
    }

    pub fn create_n_neurons_with<F>(&mut self, n: usize, f: &mut F) -> Vec<NeuronId>
    where
        F: FnMut(usize) -> NeuronConfig,
    {
        (0..n).map(|i| self.create_neuron(f(i))).collect()
    }

    pub fn connect_all(
        &mut self,
        from_neurons: &[NeuronId],
        to_neurons: &[NeuronId],
        delay: SynapseDelay,
        weight: Num,
    ) {
        self.connect_all_with(from_neurons, to_neurons, &mut |_, _| Some((delay, weight)));
    }

    pub fn connect_all_with<F>(
        &mut self,
        from_neurons: &[NeuronId],
        to_neurons: &[NeuronId],
        f: &mut F,
    ) where
        F: FnMut(NeuronId, NeuronId) -> Option<(SynapseDelay, Num)>,
    {
        for &from in from_neurons {
            for &to in to_neurons {
                match f(from, to) {
                    Some((delay, weight)) => {
                        let _ = self.connect(from, to, delay, weight);
                    }
                    None => {}
                }
            }
        }
    }

    pub fn connect(
        &mut self,
        pre_neuron: NeuronId,
        post_neuron: NeuronId,
        synapse_delay: SynapseDelay,
        weight: Num,
    ) -> SynapseId {
        assert!(pre_neuron.index() < self.neurons.len());
        assert!(post_neuron.index() < self.neurons.len());

        let synapse = Synapse {
            pre_neuron,
            post_neuron,
            synapse_delay,
            weight,
            eff_d: 0.0,
        };
        let synapse_id = SynapseId::from(self.synapses.len());

        self.synapses.push(synapse);
        self.neurons[pre_neuron.index()]
            .post_synapses
            .push(synapse_id);
        self.neurons[post_neuron.index()]
            .pre_synapses
            .push(synapse_id);

        return synapse_id;
    }

    pub fn into_network(self) -> Network {
        let NetworkBuilder { neurons, synapses } = self;

        Network { neurons, synapses }
    }
}

/*
#[test]
fn test_network() {
    let mut network = Network::new();
    let n1 = network.create_neuron(NeuronConfig::regular_spiking());
    let n2 = network.create_neuron(NeuronConfig::regular_spiking());
    assert_eq!(2, network.total_neurons());

    let _syn1 = network.connect(n1, n2, 3, 1.0);
    assert_eq!(0, network.get_neuron(n1).pre_synapses.len());
    assert_eq!(1, network.get_neuron(n1).post_synapses.len());
}
*/
