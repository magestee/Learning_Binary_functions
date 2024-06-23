use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::{self, Write};

#[derive(Serialize)]
struct DataSet {
    mu: usize,
    inputs: Vec<Vec<i32>>,
    output: Vec<i32>,
}

#[derive(Serialize)]
struct DataCollection {
    data_sets: Vec<DataSet>,
}

fn main() -> io::Result<()> {
    let n = 2; // number of bits in each vector
    let number_of_inputs = 2_usize.pow(n as u32); // 2^n, producing vectors of size 4
    let number_of_outputs = 2_usize.pow(number_of_inputs as u32); // Using the same formula for outputs

    let inputs = generate_binary_vectors(n); // Generates all combinations for inputs
    let outputs = generate_binary_vectors(number_of_inputs); // Generates all combinations for outputs
    let mut datasets = Vec::with_capacity(number_of_outputs);

    for (i, output) in outputs.iter().enumerate() {
        datasets.push(DataSet {
            mu: i + 1,
            inputs: inputs.clone(),
            output: output.clone(),
        });
    }

    // Create a collection of all datasets
    let data_collection = DataCollection { data_sets: datasets };

    // Serialize to JSON and write to file
    let file_path = "data_sets.json";
    write_to_json_file(&data_collection, file_path)?;
    Ok(())
}

fn generate_binary_vectors(n: usize) -> Vec<Vec<i32>> {
    let num_combinations = 2_usize.pow(n as u32); // 2^n combinations
    let mut combinations = Vec::with_capacity(num_combinations);

    for i in 0..num_combinations {
        let mut combination = Vec::with_capacity(n);
        for j in 0..n {
            if (i & (1 << j)) != 0 {
                combination.insert(0,1);
            } else {
                combination.insert(0, -1);
            }
        }
        combinations.push(combination);
    }

    combinations
}

fn write_to_json_file(data: &DataCollection, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    let json_data = serde_json::to_string_pretty(&data)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}
