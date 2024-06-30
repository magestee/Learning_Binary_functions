// model.rs
type Weights = Vec<Vec<Vec<f32>>>;
type Bias = Vec<Vec<f32>>;

use crate::generate_data_set::DataSet;
use core::f32;
use std::f32::consts::E;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub struct NeuralNetwork {
    pub weights: Weights,
    pub bias: Bias,
    pub weights_empty: Weights,
    pub bias_empty: Bias
}

impl NeuralNetwork {
    pub fn new(i: usize, h: usize, o: usize) -> Self {
        let mut ih_w = vec![vec![0.0; i]; h];
        let mut ho_w = vec![vec![0.0; h]; o];
        let mut bias = vec![vec![0.0; h], vec![0.0; o]]; 

        let weights_empty = vec![ih_w.clone(), ho_w.clone()];
        let bias_empty = bias.clone();

        NeuralNetwork::randomly_populate(&mut ih_w);
        NeuralNetwork::randomly_populate(&mut ho_w);
        NeuralNetwork::randomly_populate(&mut bias);  

        let weights: Weights = vec![ih_w, ho_w];  
        
        NeuralNetwork { weights, bias, weights_empty, bias_empty}
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
    
    // TODO: needs to calculate for all the layers.
    pub fn feedforward(&self, weights: Vec<Vec<f32>>, biases: Vec<f32>, inputs: Vec<f32>) -> Vec<f32> {
        let mut n = 0.0;
        let mut a: Vec<f32> = Vec::new();
        for wl in weights.iter(){
            for (i,w) in wl.iter().zip(inputs.iter()){
                n += w * i; }
            a.push(n);
            n = 0.0;
        };
        a = a.iter().zip(biases.iter()).map(|(n , b)| NeuralNetwork::sigmoid(n + b)).collect();
        a
    }

    #[allow(unused)]
    pub fn sgd(&self, inputs: Vec<Vec<f32>>, outputs: Vec<f32>, mini_batch_size: usize, epoch: usize) -> Vec<(Vec<f32>, f32)> {
        let mut pairs: Vec<_> = inputs.into_iter().zip(outputs.into_iter()).collect();

        let mut rng = thread_rng();

        pairs.shuffle(&mut rng);
        
        let mini_batch: Vec<_> = pairs.into_iter().take(mini_batch_size).collect();

        mini_batch
    }
    //TODO: must add a function called update_mini_batch that will do the backprop 
    //and updates the mini batch.

    #[allow(unused)]
    pub fn backprop(&mut self, x: Vec<f32>, y: f32) {
        let nebula_iw = self.weights_empty.clone();
        let nebula_b = self.bias_empty.clone();
        
        let activation = x;
        let activations: Vec<f32> = Vec::new();
        let zs: Vec<f32> = Vec::new();

    }
}

pub fn process_dataset(dataset: &DataSet, n: usize){
    let mut matrix: NeuralNetwork = NeuralNetwork::new(n, 5, 3);

    println!("h_w: {:?}", matrix.weights[0]);
    println!("o_w: {:?}", matrix.weights[1]);
    println!("b: {:?}", matrix.bias);
    
    println!("inputs: {:?}", dataset.inputs);
    println!("outputs: {:?}", dataset.output);

    let f = matrix.feedforward(matrix.weights[0].clone(), matrix.bias[0].clone(), dataset.inputs[2].clone());

    println!("{:?}", f);

    let mini_batch = matrix.sgd(dataset.inputs.clone(), dataset.output.clone(), 1, 100);

    matrix.backprop(mini_batch[0].0.clone(), mini_batch[0].1.clone());
    print!("{:?}", mini_batch)
}
