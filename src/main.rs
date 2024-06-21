use std::usize;

fn main() {
   let n = 8_usize;
   let m = 2_usize.pow(n as u128);

   let inputs = generate_binary_vectors(n);
   let outputs = generate_binary_vectors(m);

   for input in &inputs {
       println!("{:?}", input)
   }
   for output in &outputs {
       println!("{:?}", output)
   }
}

struct TrainingSet {
    mu: i32,
    set: Vec<(Vec<i32>, i32)>,
}

fn generate_binary_vectors(n: usize) -> Vec<Vec<i32>> {
    let num_combinations = 2_usize.pow(n as u32);
    let mut outputs = Vec::with_capacity(num_combinations); 

    for i in 0..num_combinations {
        let mut combination = Vec::with_capacity(n as usize);
        for j in 0..n {
            if ( i & ( 1 << j ) ) != 0 {
                combination.insert(0,1);
            } else {
                combination.insert(0, -1);
            }
        }
        outputs.push(combination);
    }
    
    outputs
}
/*
fn genrate_the_trainingset(n: Vec<Vec<i32>>) -> Vec<TrainingSet>{
    let training_set = Vec::new();
    let outputs = Vec::with_capacity(n.len());

    for (index, value) in n.iter().enumerate() {
        println!("{}:{:?}",index, value)
    }
}
*/
