// This is a port of Izhikevich's spnet.m as given in the article
//
// Polychronization: Computing with Spikes (2006).
//

extern crate rand;

const M: usize = 100;       // number of synapses per neuronn
const D: usize = 20;        // maximal conduction delay
const M_D: usize = M / D;   // must be an integer. number of synapses per delay step
const Ne: usize = 800;      // total number of excitatory neurons
const Ni: usize = 200;      // total number of inhibitory neurons
const N: usize = Ne + Ni;   // total number of neurons

type Num = f32;

fn main() {
    use rand::distributions::{IndependentSample, Range};

    let mut rng = rand::thread_rng();
    // exhibitory neurons connect to both kinds
    let ne_between = Range::new(0, N);
    // inhibitory neurons only connect to excitatory neurons
    let ni_between = Range::new(0, Ne);

    // neuron parameters
    let a: Vec<Num> = (0..N)
                          .map(|i| {
                              if i < Ne {
                                  0.02
                              } else {
                                  0.1
                              }
                          })
                          .collect();
    let d: Vec<Num> = (0..N)
                          .map(|i| {
                              if i < Ne {
                                  8.0
                              } else {
                                  2.0
                              }
                          })
                          .collect();

    // these are fixed parameters
    let b: Vec<Num> = (0..N).map(|_| 0.2).collect();
    let c: Vec<Num> = (0..N).map(|_| -65.0).collect();

    // maximal synaptic strenght
    let sm: Num = 10.0;

    // Indices of postsynaptic target neurons.
    // Every neuron has M postsynaptic neurons.
    let post: Vec<Vec<usize>> = (0..N)
                                    .map(|i| {
                                        if i < Ne {
                                            (0..M)
                                                .map(|_| ne_between.ind_sample(&mut rng))
                                                .collect()
                                        } else {
                                            (0..M)
                                                .map(|_| ni_between.ind_sample(&mut rng))
                                                .collect()
                                        }
                                    })
                                    .collect();

    // synaptic weights. NxM matrix
    let s: Vec<Vec<Num>> = (0..N)
                               .map(|i| {
                                   if i < Ne {
                                       (0..M).map(|_| 6.0).collect()
                                   } else {
                                       (0..M).map(|_| -5.0).collect()
                                   }
                               })
                               .collect();

    // derivatives of synaptic weights. NxM.
    // starts all zeroed.
    let s: Vec<Vec<Num>> = (0..N).map(|_| (0..M).map(|_| 0.0).collect()).collect();

    // contains the indices of the synapses (0..M) for each delay step
    let delays: Vec<Vec<Vec<usize>>> = (0..N)
                                           .map(|i| {
                                               if i < Ne {
                                                   (0..D)
                                                       .map(|j| (M_D * j..M_D * (j + 1)).collect())
                                                       .collect()
                                               } else {
                                                   (0..D)
                                                       .map(|j| {
                                                           if j == 0 {
                                                               (0..M).collect()
                                                           } else {
                                                               Vec::new()
                                                           }
                                                       })
                                                       .collect()
                                               }
                                           })
                                           .collect();


    println!("{:?}", a);
    println!("{:?}", post);
    println!("{:?}", s);
    println!("{:?}", delays);
}
