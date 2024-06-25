// model.rs

use crate::generate_data_set::DataSet;
use rand::prelude::*;
use std::f64::consts::E;

pub struct NeuralNetwork {
    pub ih_w: Vec<Vec<f64>>,
    pub ho_w: Vec<f64>,
    pub bias: f64,
}

impl NeuralNetwork {

    pub fn new(input_neurons: usize, hidden_neurons : usize) -> Self {
        let mut rng = thread_rng();

        // Initialize weights with smaller values to prevent saturation of sigmoid at the start
        let ih_w = (0..input_neurons)
            .map(|_| (0..hidden_neurons).map(|_| rng.gen_range(-0.1..0.1)).collect())
            .collect();

        // Ensure hidden_weights are initialized to match the number of hidden neurons
        let ho_w = (0..hidden_neurons).map(|_| rng.gen_range(-0.1..0.1)).collect();
        let bias = rng.gen_range(-0.1..0.1);

        NeuralNetwork {
            ih_w,
            ho_w,
            bias,
        }
    }

    pub fn sigmoid(x: f64) -> f64 {
        1.0 / (2.0 + E.powf(-x))
    }

    pub fn sigmoid_derivative(x: f64) -> f64 {
        Self::sigmoid(x) * (1.0 - Self::sigmoid(x))
    }

    pub fn train(&mut self, inputs: &[Vec<i32>], outputs: &[i32], epochs: usize) {
        let learning_rate = 0.001;
        for _ in 0..epochs {
            for (input, &target) in inputs.iter().zip(outputs) {
                let input_f64: Vec<f64> = input.iter().map(|&x| x as f64).collect();

                let hidden_inputs: Vec<f64> = self.ih_w.iter().map(|weights| {
                    let weighted_sum: f64 = weights.iter()
                                                   .zip(&input_f64)
                                                   .map(|(w, i)| w * i)
                                                   .sum();
                    weighted_sum + self.bias
                }).collect();

                let hidden_outputs: Vec<f64> = hidden_inputs.iter().map(|&x| Self::sigmoid(x)).collect();

                let final_input: f64 = hidden_outputs.iter()
                                                     .zip(&self.ho_w)
                                                     .map(|(o, w)| o * w)
                                                     .sum();

                let prediction: f64 = Self::sigmoid(final_input);

                let error: f64 = target as f64 - prediction;

                for (i, output) in hidden_outputs.iter().enumerate() {
                    if i < self.ho_w.len() {
                        self.ho_w[i] += learning_rate * error * Self::sigmoid_derivative(final_input) * output;
                    }
                }

                // Update input_weights
                for (i, input_val) in input_f64.iter().enumerate() {
                    for (j, weight) in self.ih_w[i].iter_mut().enumerate() {
                        if j < hidden_outputs.len() {
                            *weight += learning_rate * error * Self::sigmoid_derivative(hidden_inputs[j]) * input_val * hidden_outputs[j];
                        }
                    }
                }

                // Update bias
                self.bias += learning_rate * error * Self::sigmoid_derivative(final_input);
            }
        }
    }

    pub fn predict(&self, input: &[i32]) -> i32 {
        let input_f64: Vec<f64> = input.iter().map(|&x| x as f64).collect();
        let hidden_inputs: Vec<f64> = self.ih_w.iter().map(|weights| {
            weights.iter().zip(&input_f64).map(|(w, i)| w * i).sum::<f64>() + self.bias
        }).collect();
        let hidden_outputs: Vec<f64> = hidden_inputs.iter().map(|&x| Self::sigmoid(x)).collect();
        let final_input: f64 = hidden_outputs.iter().zip(&self.ho_w).map(|(o, w)| o * w).sum();
        let sigmoid_output = Self::sigmoid(final_input);
        
        if sigmoid_output >= 0.5 {
            1
        } else {
            0
        }
    }
}

pub fn process_dataset(dataset: &DataSet) -> f64 {
    /*
    println!("Processing DataSet with mu = {}", dataset.mu);
    for (input, output) in dataset.inputs.iter().zip(&dataset.output) {
        println!("Input: {:?}, Output: {}", input, output);
    }
    */
    let mut nn = NeuralNetwork::new(4, 18); // 4 inputs, 18 neurons in hidden layer
    let train_inputs = &dataset.inputs;
    let train_outputs = &dataset.output;

    nn.train(&train_inputs, &train_outputs, 1000);

    let mut correct_predictions = 0;

    // Iterate over inputs and outputs together
    for (input, &expected) in train_inputs.iter().zip(train_outputs.iter()) {
        let predicted = nn.predict(input);
        //println!("Predicted: {}, Expected: {}", predicted, expected);
        if predicted == expected {
            correct_predictions += 1;
        }
    }

    let accuracy = correct_predictions as f64 / train_inputs.len() as f64 * 100.0;
    //println!("Test Accuracy: {:.2}%", accuracy);
    accuracy // Return the accuracy here
}
