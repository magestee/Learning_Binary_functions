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

pub fn randomly_populate(a: &mut Vec<Vec<f32>>) {
    let mut rng = rand::thread_rng();
    for row in a.iter_mut() {
        for element in row.iter_mut() {
            *element = rng.gen_range(0.0..1.0);  // Using gen_range to specify the range, typical for NN weights/biases initialization.
        }
    }
}

pub fn sigmoid(z: f32) -> f32 {
    let beta = 1.0;
    let p = -beta * z;
    1.0/(1.0 + E.powf(p))
}

pub fn new_network(i: usize, h: usize, o: usize) -> NeuralNetwork {
    let mut ih_w: Weights = vec![vec![0.0; i]; h];
    let mut ho_w: Weights = vec![vec![0.0; h]; o];
    let mut bias: Bias = vec![vec![0.0; h], vec![0.0; o]]; // Ensuring the correct structure for biases.

    randomly_populate(&mut ih_w);
    randomly_populate(&mut ho_w);
    randomly_populate(&mut bias);  // Now correctly populating biases since bias is Vec<Vec<f32>>

    NeuralNetwork { ih_w, ho_w, bias }
}
pub fn feedforward(weights: Weights, biases: Bias, inputs: Vec<f32>) {
    let mut n = 0.0;
    let mut a: Vec<f32> = Vec::new();
    for wl in weights.iter(){
        for (i,w) in wl.iter().zip(inputs.iter()){
            println!("i: {}, w: {}", i,w);
            n += w * i;
            println!("{}", n)
        }
        a.push(n);
        n = 0.0;
        println!("a= {:?}", a)
    }
}

pub fn process_dataset(dataset: &DataSet, n: usize){
    let matric: NeuralNetwork = new_network(n, 3, 1);
    println!("h_w: {:?}", matric.ih_w);
    println!("o_w: {:?}", matric.ho_w);
    println!("b: {:?}", matric.bias);

    println!("inputs: {:?}", dataset.inputs);
    println!("outputs: {:?}", dataset.output);

    let f = feedforward(matric.ih_w.clone(), matric.bias.clone(), dataset.inputs[2].clone());

    println!("{:?}", f);
}
