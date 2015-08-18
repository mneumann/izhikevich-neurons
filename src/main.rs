extern crate izhikevich_neurons;
extern crate gnuplot;

use izhikevich_neurons::{Config, State, Num};
use gnuplot::{Figure, Caption, Color, AxesCommon};

fn main() {
    let config = Config::regular_spiking();

    const PARAMS : &'static [(&'static str, Num, &'static str)] = &[
        ("Neuron 1 [-7 uA current 200..700ms]", -7.0, "blue"),
        ("Neuron 2 [-2.69 uA current 200..700ms]", -2.69, "red"),
        ("Neuron 3 [-2.7 uA current 200..700ms]", -2.7, "green")
    ];

    let mut neurons: Vec<_> = PARAMS.iter().map(|_| State::new()).collect();
    let mut potentials: Vec<_> = PARAMS.iter().map(|_| Vec::new()).collect();

    let mut time = 0.0;
    let mut times = Vec::new();

    while time < 1_000.0 {
        // record current state
        times.push(time);
        for (i, &mut neuron) in neurons.iter_mut().enumerate() {
            potentials[i].push(neuron.potential());
        }
        time += 1.0;

        // update state
        for (i, mut neuron) in neurons.iter_mut().enumerate() {
            let syn_i = if time >= 200.0 && time <= 700.0 { PARAMS[i].1 } else { 0.0 };
            *neuron = neuron.step_1ms(syn_i, &config);
        }
    }

    let mut fg = Figure::new();
    {
        let mut diag = fg.axes2d().
            set_x_label("time [ms]", &[]).
            set_y_label("potential [mV]", &[]);
        for (i, &p) in PARAMS.iter().enumerate() {
            diag.lines(times.iter(), potentials[i].iter(), &[Caption(p.0), Color(p.2)]);
        }
    }
    fg.show();
}
