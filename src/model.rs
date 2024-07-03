// model.rs
type Weights = Vec<Vec<Vec<f32>>>;
type Biases = Vec<Vec<f32>>;
type Inputs = Vec<Vec<f32>>;
type Outputs = Vec<f32>;

use crate::generate_data_set::DataSet;
use core::f32;
use std::f32::consts::E;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

#[allow(unused)]
pub struct NeuralNetwork {
    pub inputs: Inputs,
    pub outputs: Outputs,
    pub weights: Weights,
    pub biases: Biases,
    pub weights_empty: Weights,
    pub bias_empty: Biases
}

#[allow(dead_code)]
#[allow(unused)]
impl NeuralNetwork {
    pub fn new(i: usize,
               h: usize,
               o: usize,
               inputs: Inputs,
               outputs: Outputs)
               -> Self {

        let mut ih_w = vec![vec![0.0; i]; h];
        let mut ho_w = vec![vec![0.0; h]; o];
        let mut biases = vec![vec![0.0; h], vec![0.0; o]]; 

        let weights_empty = vec![ih_w.clone(), ho_w.clone()];
        let bias_empty = biases.clone();

        NeuralNetwork::randomly_populate(&mut ih_w);
        NeuralNetwork::randomly_populate(&mut ho_w);
        NeuralNetwork::randomly_populate(&mut biases);  

        let weights: Weights = vec![ih_w, ho_w];  
        
        NeuralNetwork {
            inputs,
            outputs,
            weights,
            biases,
            weights_empty,
            bias_empty
        }
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

    pub fn layerforward(inputs: Vec<f32>,
                        weights: Vec<Vec<f32>>,
                        biases: Vec<f32>) 
                        -> Vec<f32> {
        let mut n = 0.0;
        let mut a: Vec<f32> = Vec::new();
        for wl in weights.clone().into_iter(){
            println!("wl: {:?}", wl);
            for (i,w) in wl.iter().zip(inputs.iter()){
                n += w * i; 
            }
            a.push(n);
            n = 0.0;
        };
        a = a.iter().zip(biases.iter())
            .map(|(n , b)| NeuralNetwork::sigmoid(n + b))
            .collect();
        a
    }

    pub fn feedforward(&mut self) {
        let a: Vec<Vec<f32>> = Vec::new();
        
        print!("{:?}", a)
    }
    

    pub fn sgd(&self,
               inputs: Vec<Vec<f32>>,
               outputs: Vec<f32>,
               mini_batch_size: usize,
               epoch: usize)
               -> Vec<(Vec<f32>, f32)> {

        let mut pairs: Vec<_> = inputs
            .into_iter()
            .zip(outputs.into_iter())
            .collect();

        let mut rng = thread_rng();

        pairs.shuffle(&mut rng);
        
        let mini_batch: Vec<_> = pairs
            .into_iter()
            .take(mini_batch_size)
            .collect();

        mini_batch
    }
    //TODO: must add a function called update_mini_batch that will do the backprop 
    //and updates the mini batch.

    pub fn backprop(&mut self, x: Vec<f32>, y: f32) {
        let nebula_iw = self.weights_empty.clone();
        let nebula_b = self.bias_empty.clone();
        
        let activation = x;
        let activations: Vec<f32> = Vec::new();
        let zs: Vec<f32> = Vec::new();

    }
}

pub fn process_dataset(dataset: &DataSet, n: usize){
    let mut matrix: NeuralNetwork = 
        NeuralNetwork::new(n, 5, 3, dataset.inputs.clone(), dataset.output.clone());
    
    /*
    println!("inputs: {:?}", dataset.inputs);
    println!("outputs: {:?}", dataset.output);

    println!("w: {:?}", matrix.weights);
    println!("b: {:?}", matrix.biases);
    */

    let f = matrix.feedforward();

    println!("f: {:?}", f);
    
    /*
    let mini_batch = matrix.sgd(dataset.inputs.clone(), dataset.output.clone(), 1, 100);

    matrix.backprop(mini_batch[0].0.clone(), mini_batch[0].1.clone());
    print!("{:?}", mini_batch)
    */
}
