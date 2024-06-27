// model.rs
type Weights = Vec<Vec<f32>>;
type Bias = Vec<Vec<f32>>;

use crate::generate_data_set::DataSet;
use std::f32::consts::E;
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

fn dot_product(v1: &[f64], v2: &[f64]) -> f64 {
    v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum()
}

pub fn sigmoid(z: f32) -> f32 {
    let beta = 1.0;
    let p = -beta * z;
    1.0/(1.0 + E.powf(p))
}

pub fn sigmoid_derivative(z:f32, beta: f32) -> f32 {
    let s = sigmoid(z);
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

pub fn feedforward(w_vec: Vec<f32>, b_vec: Vec<f32>, mut a: Vec<f32>) -> Vec<f32> {
    for (w,b) in w_vec.iter().zip(b_vec.iter()) {
        a  = a.iter().map(|ai| sigmoid(w * ai + b)).collect();
    };
    a
}

pub fn process_dataset(dataset: &DataSet, n: usize){
    let matric: NeuralNetwork = new_network(n, 5, 1);
    println!("h_w: {:?}", matric.ih_w);
    println!("o_w: {:?}", matric.ho_w);
    println!("b: {:?}", matric.bias);

    println!("inputs: {:?}", dataset.inputs);
    println!("outputs: {:?}", dataset.output);


    let f = feedforward(matric.ih_w[0].clone(), matric.bias[0].clone(), dataset.inputs[0].clone());
    println!("{:?}", f)
}
