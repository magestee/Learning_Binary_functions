// model.rs
type Weights = Vec<Vec<Vec<f32>>>;
type Bias = Vec<Vec<f32>>;

use crate::generate_data_set::DataSet;
use core::f32;
use std::f32::consts::E;
//use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub struct NeuralNetwork {
    pub weights: Weights,
    pub bias: Bias,
}

pub fn randomly_populate(a: &mut Vec<Vec<f32>>) {
    let mut rng = rand::thread_rng();
    for row in a.iter_mut() {
        for element in row.iter_mut() {
            *element = rng.gen_range(0.0..1.0);
        }
    }
}

pub fn sigmoid(z: f32) -> f32 {
    let beta = 1.0;
    let p = -beta * z;
    1.0/(1.0 + E.powf(p))
}

pub fn new_network(i: usize, h: usize, o: usize) -> NeuralNetwork {
    let ih_w = vec![vec![0.0; i]; h];
    let ho_w = vec![vec![0.0; h]; o];

    let weights: Weights = vec![ih_w, ho_w];  
    
    let bias: Bias = vec![vec![0.0; h], vec![0.0; o]]; 

    /*
    randomly_populate(&mut matric.ih_w);
    randomly_populate(&mut matric.ho_w);
    randomly_populate(&mut matric.bias);  
    */


    NeuralNetwork { weights, bias }
}

pub fn feedforward(weights: Vec<Vec<f32>>, biases: Vec<f32>, inputs: Vec<f32>) -> Vec<f32> {
    let mut n = 0.0;
    let mut a: Vec<f32> = Vec::new();
    for wl in weights.iter(){
        for (i,w) in wl.iter().zip(inputs.iter()){
            n += w * i;
        }
        a.push(n);
        n = 0.0;
    };
    a = a.iter().zip(biases.iter()).map(|(n , b)| sigmoid(n + b)).collect();
    a
}

/*
pub fn sgd(inputs: Vec<Vec<f32>>, outputs: Vec<f32>, mini_batch_size: usize, epoch: usize) {
    let mut pairs: Vec<_> = inputs.iter().zip(outputs.iter()).collect();

    let mut rng = thread_rng();

    pairs.shuffle(&mut rng);
    
    let mini_batch = &mut pairs[0..mini_batch_size];

    println!("{:?}", mini_batch)
}

pub fn backprop(io: Vec<(Vec<f32>, f32)>, network:&mut NeuralNetwork) {
    let nebula_iw = network.ih_w;
    let nebula_hw = network.ho_w;
    let nebula_b = network.bias;
    
    let layer_one_activation = 
}
*/

pub fn process_dataset(dataset: &DataSet, n: usize){
    let matric: NeuralNetwork = new_network(n, 3, 1);

    println!("h_w: {:?}", matric.weights[0]);
    println!("o_w: {:?}", matric.weights[0]);
    println!("b: {:?}", matric.bias);
    
    println!("inputs: {:?}", dataset.inputs);
    println!("outputs: {:?}", dataset.output);

    let f = feedforward(matric.weights[0].clone(), matric.bias[0].clone(), dataset.inputs[2].clone());

    println!("{:?}", f);

    //sgd(dataset.inputs.clone(), dataset.output.clone(), 1, 100);
}
