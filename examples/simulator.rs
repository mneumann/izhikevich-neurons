use gnuplot::{AutoOption, AxesCommon, /*Caption,*/ Color, Figure, PlotOption};
use izhikevich_neurons::model::{NeuronConfig, StdpConfig};
use izhikevich_neurons::network::{NetworkBuilder, NeuronId, SynapseDelay};
use izhikevich_neurons::simulation::{FireRecorder, Simulator, Timestep};
use izhikevich_neurons::Num;

fn main() {
    // const PARAMS : &'static [(&'static str, &'static str)] = &[
    // ("Neuron 1 [7 pA current 200..700ms]", "blue"),
    // ("Neuron 2 [2.69 pA current 200..700ms]", 2.69, "red"),
    // ("Neuron 2 [0.0 pA current 200..700ms]", "red"),
    // ("Neuron 3 [2.7 pA current 200..700ms]", 2.7, "green")
    // ];
    //

    let mut fire_recorder = FireRecorder::new();
    let mut builder = NetworkBuilder::new();

    let input_neurons = builder.create_n_neurons_with(9, &mut |_| NeuronConfig::regular_spiking());
    let middle_neurons =
        builder.create_n_neurons_with(18, &mut |_| NeuronConfig::regular_spiking());
    let output_neurons = builder.create_n_neurons_with(1, &mut |_| NeuronConfig::regular_spiking());

    // connect every input neuron with every output neuron.
    builder.connect_all(
        &input_neurons[..],
        &middle_neurons[..],
        SynapseDelay::new(2),
        2.0,
    );
    builder.connect_all(
        &input_neurons[..],
        &middle_neurons[..],
        SynapseDelay::new(3),
        2.0,
    );
    builder.connect_all(
        &input_neurons[..],
        &middle_neurons[..],
        SynapseDelay::new(5),
        2.0,
    );
    builder.connect_all(
        &middle_neurons[..],
        &output_neurons[..],
        SynapseDelay::new(1),
        1.0,
    );

    let pattern1: [u8; 9] = [0, 0, 0, 1, 1, 1, 0, 0, 0];

    let pattern2: [u8; 9] = [0, 0, 1, 0, 1, 0, 1, 0, 0];

    let mut external_inputs: Vec<(NeuronId, Timestep, Num)> = Vec::new();

    for (i, &v) in pattern1.iter().enumerate() {
        external_inputs.push((input_neurons[i], 0, if v == 0 { 0.0 } else { 4.0 }));
        external_inputs.push((input_neurons[i], 2000, if v == 0 { 0.0 } else { 4.0 }));
        external_inputs.push((input_neurons[i], 4000, if v == 0 { 0.0 } else { 4.0 }));
        external_inputs.push((input_neurons[i], 6000, if v == 0 { 0.0 } else { 4.0 }));
        external_inputs.push((input_neurons[i], 7000, if v == 0 { 0.0 } else { 4.0 }));
    }

    for (i, &v) in pattern2.iter().enumerate() {
        external_inputs.push((input_neurons[i], 1000, if v == 0 { 0.0 } else { 4.0 }));
        external_inputs.push((input_neurons[i], 3000, if v == 0 { 0.0 } else { 4.0 }));
        external_inputs.push((input_neurons[i], 5000, if v == 0 { 0.0 } else { 4.0 }));
        external_inputs.push((input_neurons[i], 6500, if v == 0 { 0.0 } else { 4.0 }));
        external_inputs.push((input_neurons[i], 7500, if v == 0 { 0.0 } else { 4.0 }));
    }

    let mut network = builder.into_network();
    let mut sim = Simulator::new(network.max_synapse_delay(), StdpConfig::default());

    while sim.current_time_step() <= 10_000 {
        // set external inputs

        for &(neuron_id, at, current) in external_inputs.iter() {
            if sim.current_time_step() == at {
                network.set_external_input(neuron_id, current);
            }
        }

        sim.step(&mut network, &mut |neuron_id, timestep| {
            fire_recorder.record(neuron_id, timestep);
        });

        if sim.current_time_step() % 500 == 0 {
            // Update synapse weights every 10 ms
            if sim.current_time_step() < 6001 {
                network.update_synapse_weights(0.0, 10.0, 0.9);
            }
        }
    }

    {
        // println!("{:?}", fire_recorder);
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

    // {
    // let mut fg = Figure::new();
    // {
    // let mut diag = fg.axes2d().
    // set_x_label("time (ms)", &[]).
    // set_y_label("membrane potential v (mV)", &[]);
    // for (i, &p) in PARAMS.iter().enumerate() {
    // diag.lines(states[i].iter().enumerate().map(|(i, _)| i as f32), states[i].iter().map(|s| s.potential()), &[Caption(p.0), Color(p.1)]);
    // }
    // }
    // fg.show();
    // }
    //

    // let mut fg = Figure::new();
    // {
    // let mut diag = fg.axes2d().
    // set_x_label("membrane potential v (mV)", &[]).
    // set_y_label("recovery variable u", &[]);
    // for (i, &p) in PARAMS.iter().enumerate() {
    // diag.lines(states[i].iter().map(|s| s.potential()), states[i].iter().map(|s| s.recovery()), &[Caption(p.0), Color(p.2)]);
    // }
    // }
    // fg.show();
    //
}
