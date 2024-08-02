// model.rs
//TODO:
// - write a utility function for transpose.
// - write a utilitu function for dot product.
// - use wrapping sub for l-1 in backprop code.
// TYPE DEFINITIONS:
type Outputs = Vec<f32>;
type Biases = Vec<Vec<f32>>;
type Inputs = Vec<Vec<f32>>;
type Weights = Vec<Vec<Vec<f32>>>;

// LIBRARIES:
use core::f32;
use std::f32::consts::E;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use crate::generate_data_set::DataSet;

// OBJECT OREINTED BUT NOT REALLY OBJECT OREINTED!
#[allow(unused)]
pub struct NeuralNetwork {
    pub inputs: Inputs,
    pub outputs: Outputs,
    pub weights: Weights,
    pub biases: Biases,
    pub weights_empty: Weights,
    pub bias_empty: Biases,
    pub input_neurons: usize,
    pub hidden_neurons: usize,
    pub output_neurons: usize,
    pub beta: f32
}

#[allow(dead_code)]
#[allow(unused)]
impl NeuralNetwork {
    // THE FUNCTION RESPONSIBLE FOR CREATING THE NEURAL NET.
    pub fn new(i: usize,
               h: usize,
               o: usize,
               inputs: Inputs,
               outputs: Outputs,
               beta: f32)
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
            bias_empty,
            input_neurons: i,
            hidden_neurons: h,
            output_neurons: o,
            beta,
        }
    }

    // FUNCTION THAT RANDOMLY POPULATES THE NEW NET WEIGHT MATRIX
    // WITH RANDOM VALUES.
    pub fn randomly_populate(a: &mut Vec<Vec<f32>>) {
        let mut rng = rand::thread_rng();
        for row in a.iter_mut() {
            for element in row.iter_mut() {
                *element = rng.gen_range(0.0..1.0);
            }
        }
    }

    pub fn transpose(matrix: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
       let rows = matrix.len();
       let cols = matrix[0].len();
       let mut transpose = vec![vec![0.0; rows]; cols];

       for i in 0..rows {
           for j in 0..cols {
               transpose[j][i] = matrix[i][j];
           }
       }

       transpose
    }

    pub fn dot(a: Vec<f32>, b: Vec<f32>) -> f32 {
        let mut o = 0.0;
        for (i, j) in a.iter().zip(b.iter()) {
            o = i * j
        }
        o
    }

    //SIGMOID FUNCTION AS OUR ACTIVATION FUNCTION
    pub fn sigmoid(&self, z: f32) -> f32 {
        let p = -self.beta * z;
        return 1.0/(1.0 + E.powf(p))
    }

    //THE DERIVATIVE OF THE ACTIVATION FUNCTION
    pub fn sigmoid_prime(&self,z: f32) -> f32 {
       return  self.sigmoid(z) * (1.0 - self.sigmoid(z));
    }

    //THE FUNCTION THAT CALCULATES A LAYER OF FEEDFORWARD
    pub fn layerforward(&self, inputs: Vec<f32>,
                        weights: Vec<Vec<f32>>,
                        biases: Vec<f32>) 
                        -> Vec<f32> {

        let mut n = 0.0;
        let mut a: Vec<f32> = Vec::new();
        for wl in weights.clone().into_iter(){
            for (i,w) in wl.iter().zip(inputs.iter()){
                n += w * i; 
            }
            a.push(n);
            n = 0.0;
        };
        a = a.iter().zip(biases.iter())
            .map(|(n , b)| self.sigmoid(n + b))
            .collect();
        a
    }

    //FUNCTION THAT CAlCULATES FEEDFORWARD THROUGH THE NET
    //ONE LAYER AT A TIME, USING layerforward
    pub fn feedforward(&mut self,
                       inputs: Vec<f32>)
                       -> Vec<Vec<f32>> {

        let mut a: Vec<Vec<f32>> = Vec::new();
        a.insert(0, inputs);
        for i in 0..self.weights.len(){
            a.push(self.layerforward(a[i].clone(),
                                    self.weights[i].clone(), 
                                    self.biases[i].clone()));
        }
        a
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

    pub fn updates_mini_batch(&mut self) {
        let nebula_iw = self.weights_empty.clone();
        let nebula_b = self.bias_empty.clone();

        for (i, o) in self.inputs.clone().iter().zip(self.outputs.clone().iter()){
            self.backprop(i.clone(), o.clone());
        }
    }

    //TODO: complete the backprop function.
    pub fn backprop(&mut self, x: Vec<f32>, y: f32) {
        let mut nebula_iw= self.weights_empty.clone();
        let mut nebula_b = self.bias_empty.clone();
        
        let zs: Vec<Vec<f32>> = self.feedforward(x);

        let mut cd: f32 = self.cost_derivative(zs[2][0].clone(), y);
        let mut sp = self.sigmoid_prime(zs[2][0]);

        let delta = cd * sp; 
        let iw = vec![NeuralNetwork::dot(vec![delta] , zs[2].clone())];

        print!(" z  {:?}", iw);

        nebula_b.insert(1, vec![delta]);
        nebula_iw.insert(2, vec![iw]);
    }

    pub fn cost_derivative(&self, z: f32, y: f32)  -> f32{
        z-y
    }
}

pub fn process_dataset(dataset: &DataSet, n: usize){
    let mut matrix: NeuralNetwork = 
        NeuralNetwork::new(n,
                           5,
                           1,
                           dataset.inputs.clone(),
                           dataset.output.clone(),
                           1.0);
    
    matrix.updates_mini_batch()
    /*
    println!("inputs: {:?}", dataset.inputs);
    println!("outputs: {:?}", dataset.output);

    println!("w: {:?}", matrix.weights);
    println!("b: {:?}", matrix.biases);
    */

    /*
    let mini_batch = matrix.sgd(dataset.inputs.clone(), dataset.output.clone(), 1, 100);

    matrix.backprop(mini_batch[0].0.clone(), mini_batch[0].1.clone());
    print!("{:?}", mini_batch)
    */
}
