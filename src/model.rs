// model.rs

use crate::generate_data_set::DataSet;
/*
pub struct NeuralNetwork {
    pub ih_w: Vec<Vec<f64>>,
    pub ho_w: Vec<f64>,
    pub bias: f64,
}

impl NeuralNetwork {

    pub fn new(input_neurons: usize, hidden_neurons : usize) -> Self {
    }

    pub fn sigmoid(x: f64) -> f64 {
    }

    pub fn sigmoid_derivative(x: f64) -> f64 {
    }

    pub fn train(&mut self, inputs: &[Vec<i32>], outputs: &[i32], epochs: usize) {
    }

    pub fn predict(&self, input: &[i32]) -> i32 {
}
*/

pub fn process_dataset(dataset: &DataSet){
    print!("{:?}",dataset.inputs)
}
