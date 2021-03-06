use gnuplot::{AutoOption, AxesCommon, /* Caption, */ Color, Figure, PlotOption};
use izhikevich_neurons::model::{NeuronConfig, StdpConfig};
use izhikevich_neurons::network::{NetworkBuilder, SynapseDelay};
use izhikevich_neurons::simulation::{Event, EventQueue, FireRecorder, Simulator};
use izhikevich_neurons::Closed01;
use rand::{seq::SliceRandom, Rng};

const INPUTS: usize = 9;

fn main() {
    let mut rng = rand::thread_rng();
    let mut fire_recorder = FireRecorder::new();
    let mut builder = NetworkBuilder::new();

    let input_neurons = builder.create_n_neurons_with(INPUTS, &mut |_| NeuronConfig::chattering());
    let hidden_neurons = builder.create_n_neurons_with(18, &mut |_| {
        NeuronConfig::excitatory(Closed01::new(rng.gen()))
    });
    let _output_neurons =
        builder.create_n_neurons_with(1, &mut |_| NeuronConfig::regular_spiking());

    for _ in 1..10 {
        builder.connect_all_with(&input_neurons, &hidden_neurons, &mut |_, _| {
            Some((
                SynapseDelay::new(rng.gen_range(2, 56)),
                rng.gen_range(0.0, 5.0),
            ))
        });
    }

    let mut network = builder.into_network();

    let mut sim = Simulator::new(network.max_synapse_delay(), StdpConfig::default());

    let mut external_inputs = EventQueue::new();

    for _ in 1..2000 {
        let event = Event {
            at: rng.gen_range(1, 10_000),
            neuron: input_neurons.choose(&mut rng).cloned().unwrap(),
            weight: rng.gen_range(0.0, 4.0),
        };
        let event_down = Event {
            at: event.at + rng.gen_range(2, 10),
            neuron: event.neuron,
            weight: -event.weight,
        };

        external_inputs.push(event);
        external_inputs.push(event_down);
    }

    while sim.current_time_step() <= 10_000 {
        while let Some(ev) = external_inputs.pop_next_event_at(sim.current_time_step()) {
            network.increase_external_input(ev.neuron, ev.weight);
        }

        sim.step(&mut network, &mut fire_recorder);

        if sim.current_time_step() % 500 == 0 {
            // Update synapse weights every 500 ms
            network.update_synapse_weights(0.0, 10.0, 0.9);
        }
    }

    {
        let mut fg = Figure::new();
        {
            let diag = fg
                .axes2d()
                .set_y_ticks(Some((AutoOption::Fix(1.0), 0)), &[], &[])
                .set_y_range(
                    AutoOption::Fix(0.0),
                    AutoOption::Fix((network.total_neurons() - 1) as f64),
                )
                .set_x_label("time (ms)", &[])
                .set_y_label("neuron id", &[]);

            diag.points(
                fire_recorder.events.iter().map(|&(_, t)| t),
                fire_recorder.events.iter().map(|&(i, _)| i.index()),
                &[
                    PlotOption::PointSymbol('S'),
                    Color("black"),
                    PlotOption::PointSize(0.2),
                ],
            );
        }
        fg.show();
    }
}
