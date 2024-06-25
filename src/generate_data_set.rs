// generate_dataset.rs

use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
pub struct DataSet {
    pub mu: usize,
    pub inputs: Vec<Vec<i32>>,
    pub output: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct DataCollection {
    pub data_sets: Vec<DataSet>,
}

pub fn generate_data_set(bits : usize) -> io::Result<()> {
    let n = 2_usize.pow(bits as u32);
    let m = 2_usize.pow(n as u32);

    let inputs = generate_binary_vectors(bits);
    let outputs = generate_binary_vectors(n);
    let mut datasets = Vec::with_capacity(m);

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
    write_to_json_file(&data_collection, file_path)
}

fn generate_binary_vectors(n: usize) -> Vec<Vec<i32>> {
    let num_combinations = 2_usize.pow(n as u32);
    let mut combinations = Vec::with_capacity(num_combinations);

    for i in 0..num_combinations {
        let mut combination = Vec::with_capacity(n);
        for j in 0..n {
            if (i & (1 << j)) != 0 {
                combination.insert(0, 1);
            } else {
                combination.insert(0, 0);
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
