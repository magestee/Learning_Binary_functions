fn main() {
   let size = 4;

   let outputs = generate_all_binary_inputs(size);

   /*
   for output in &outputs {
       println!("{:?}", output)
   }
   for (key, value) in &outputs {
       println!("{}:{:?}", key, value);
   }
   */

   // let trainingset = genrate_the_trainingset(outputs);
   //genrate_the_trainingset(outputs)
   
   let even = make_fibonacci_num_vec(outputs);


   for (key, value) in &even {
       println!("{:?},{}", key, value)
   }

}

struct TrainingSet {
    mu: i32,
    input: Vec<i32>,
    output: Vec<i32>,
}

fn generate_all_binary_inputs(n: u32) -> Vec<Vec<i32>> {
    let num_combinations = 2_usize.pow(n);
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

    
    TrainingSet {
        mu: n.len() as i32,
        input: Vec::new(),
        output: Vec::new()
    }
}
*/

fn is_fibonacci(n: usize) -> bool {
    fn is_perfect_square(x: usize) -> bool {
        let s = (x as f64).sqrt() as usize;
        s * s == x
    }

    is_perfect_square(5 * n * n + 4) || is_perfect_square(5 * n * n - 4)
} 

fn make_even_num_vec(vector: Vec<Vec<i32>>) -> Vec<(Vec<i32>, i8)>{
    let mut outputs: Vec<(Vec<i32>, i8)> = Vec::new();
    for (index, value) in vector.iter().enumerate() {
        if index % 2 == 0 {
            outputs.push((value.to_vec(), 1))
        } else {
            outputs.push((value.to_vec(), -1))
        }
    }
    outputs
}

fn make_odd_num_vec(vector: Vec<Vec<i32>>) -> Vec<(Vec<i32>, i8)>{
    let mut outputs: Vec<(Vec<i32>, i8)> = Vec::new();
    for (index, value) in vector.iter().enumerate() {
        if index % 2 != 0 {
            outputs.push((value.to_vec(), 1))
        } else {
            outputs.push((value.to_vec(), -1))
        }
    }
    outputs
}

fn make_fibonacci_num_vec(vector: Vec<Vec<i32>>) -> Vec<(Vec<i32>, i8)> {
    let mut outputs: Vec<(Vec<i32>, i8)> = Vec::new();
    for (index, value) in vector.iter().enumerate() {
        if is_fibonacci(index) {
            outputs.push((value.to_vec(), 1))
        } else {
            outputs.push((value.to_vec(), -1))
        }
    }
    outputs
}
