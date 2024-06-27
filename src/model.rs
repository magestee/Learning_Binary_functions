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
    let mut ih_w: Weights = vec![vec![0.0; h]; i];
    let mut ho_w: Weights = vec![vec![0.0; o]; h];
    let mut bias: Bias = vec![vec![0.0; h], vec![0.0; o]]; // Ensuring the correct structure for biases.

    randomly_populate(&mut ih_w);
    randomly_populate(&mut ho_w);
    randomly_populate(&mut bias);  // Now correctly populating biases since bias is Vec<Vec<f32>>

    NeuralNetwork { ih_w, ho_w, bias }
}
/*
pub fn feedforward(weights: Weights, biases: Bias, inputs: Vec<f32>) -> Vec<f32> {
    let mut activations = inputs;

    // Iterate through each layer of weights and corresponding biases
    for (w_layer, b_layer) in weights.iter().zip(biases.iter()) {
        // Map each neuron's weights and corresponding bias to a new activation
        let new_activations: Vec<f32> = w_layer.iter().zip(b_layer.iter()).map(|(neuron_weights, &bias)| {
            // Calculate the sum of weights * inputs and add bias
            let neuron_output = neuron_weights.iter().zip(&activations)
                .map(|(&weight, &input)| weight * input) // Multiply each weight by its corresponding input
                .sum::<f32>() + bias; // Add bias for this neuron
            sigmoid(neuron_output) // Apply the sigmoid function
        })
        .collect();

        activations = new_activations; // Update activations for the next layer
    }

    activations
}
*/
pub fn process_dataset(dataset: &DataSet, n: usize){
    let matric: NeuralNetwork = new_network(n, 5, 1);
    println!("h_w: {:?}", matric.ih_w);
    println!("o_w: {:?}", matric.ho_w);
    println!("b: {:?}", matric.bias);

    println!("inputs: {:?}", dataset.inputs);
    println!("outputs: {:?}", dataset.output);

    /*
    let f = feedforward(matric.ih_w.clone(), matric.bias.clone(), dataset.inputs[0].clone());
    println!("First weight: {:?}", matric.ho_w);
    println!("First bias: {:?}", matric.bias[0]);
    println!("First input: {:?}", dataset.inputs[0]);

    println!("{:?}", f);
    */
}
