use prophet::prelude::*;
use prophet::prelude::Activation::Tanh;

fn _train(){
    let (t, f)  = (1.0, -1.0);
    // static samples are easily generated with this macro!
    let train_samples = samples![
        [f, f] => f, // ⊥ ∧ ⊥ → ⊥
        [f, t] => t, // ⊥ ∧ ⊤ → ⊤
        [t, f] => t, // ⊤ ∧ ⊥ → ⊤
        [t, t] => t  // ⊤ ∧ ⊤ → ⊤
    ];
    
    // create the topology for our neural network
    let top = Topology::input(2) // has two input neurons
        .layer(3, Tanh)          // with 3 neurons in the first hidden layer
        .layer(2, Tanh)          // and 2 neurons in the second hidden layer
        .output(1, Tanh);        // and 1 neuron in the output layer
    
    let mut net = top.train(train_samples)
        .learn_rate(0.25)    // use the given learn rate
        .learn_momentum(0.6) // use the given learn momentum
        .log_config(LogConfig::Iterations(100)) // log state every 100 iterations
        .scheduling(Scheduling::Random)         // use random sample scheduling
        .criterion(Criterion::Iterations(10000))  // train until the recent MSE is below 0.05
        .go()      // start the training session
        .unwrap(); // be ashamed to unwrap a Result
    
    // PROFIT! now you can use the neural network to predict data!
    println!("{}", net.predict(&[f, f])[0]);
    assert_eq!(net.predict(&[f, f])[0].round(), f);
    assert_eq!(net.predict(&[f, t])[0].round(), t);
}

use std::fs;
use libflate::gzip::Decoder;
use bincode::{deserialize_from, Infinite};
use train::data::TrainExample;

pub fn _load_train_data(){
    let mut file =fs::File::open("train.bin").expect("Failed to open train data dump.");
    let mut file = Decoder::new(file).unwrap();
    while let Ok(data) = deserialize_from(&mut file, Infinite){
        let data: TrainExample = data;
        println!("{:?}", data);
    }
}