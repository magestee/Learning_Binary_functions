use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::{self, Write};

#[derive(Serialize)]
struct Data {
    exponentiation_result: u32,
    binary_vectors: Vec<Vec<i32>>,
}

fn main() -> io::Result<()> {
    // Set n to 2^3
    let n = 2_usize.pow(3);
    
    // Calculate 2^n using u32
    let m = 2_u32.pow(n as u32); // 2^n

    // Generate binary vectors for n
    let inputs = generate_binary_vectors(n);

    // Create data structure for JSON serialization
    let data = Data {
        exponentiation_result: m,
        binary_vectors: inputs,
    };

    // Serialize to JSON and write to file
    let file_path = "binary_vectors.json";
    write_to_json_file(&data, file_path)?;
    Ok(())
}

fn generate_binary_vectors(n: usize) -> Vec<Vec<i32>> {
    let num_combinations = 2_usize.pow(n as u32);
    let mut outputs = Vec::with_capacity(num_combinations);

    for i in 0..num_combinations {
        let mut combination = Vec::with_capacity(n);
        for j in 0..n {
            if (i & (1 << (n - j - 1))) != 0 {
                combination.push(1);
            } else {
                combination.push(-1);
            }
        }
        outputs.push(combination);
    }

    outputs
}

fn write_to_json_file(data: &Data, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    let json_data = serde_json::to_string_pretty(&data)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}
