// model.rs
type Weights = Vec<Vec<f32>>;
type Bias = Vec<Vec<f32>>;

use crate::generate_data_set::DataSet;
use std::f64::consts::E;
use rand::Rng;

pub struct NeuralNetwork {
    pub ih_w: Weights,
    pub ho_w: Weights,
    pub bias: Bias,
}

pub fn randomly_populate(a: &mut Weights)  {
    let  mut rng = rand::thread_rng();

    for row in a.iter_mut(){
        for element in row.iter_mut(){
            *element = rng.gen();
        }
    }
}

pub fn sigmoid(z: f64, beta: f64) -> f64 {
    1.0/(1.0 + E.powf(-beta * z))
}

pub fn sigmoid_derivative(z:f64, beta: f64) -> f64 {
    let s = sigmoid(z, beta);
    beta * s * (1.0 - s)
}

pub fn new_network(i: usize, h: usize, o:usize) -> NeuralNetwork{
    let mut ih_w: Weights = vec![vec![0.0;h];i];
    let mut ho_w: Weights = vec![vec![0.0;o];h];
    let bh = vec![0.0;h];
    let bo = vec![0.0;o];

    let mut bias: Bias = [bh, bo].to_vec(); 

    randomly_populate(&mut ih_w);
    randomly_populate(&mut ho_w);
    randomly_populate(&mut bias);

    NeuralNetwork {
        ih_w,  
        ho_w,
        bias
    }

}
/*

impl NeuralNetwork {
    pub fn train(&mut self, inputs: &[Vec<i32>], outputs: &[i32], epochs: usize) {
    }
    pub fn predict(&self, input: &[i32]) -> i32 {
}
*/

pub fn process_dataset(dataset: &DataSet, n: usize){
    let matric: NeuralNetwork = new_network(n, 5, 2);
    println!("h_w: {:?}", matric.ih_w);
    println!("o_w: {:?}", matric.ho_w);
    println!("b: {:?}", matric.bias);

}
