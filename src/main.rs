use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Set n to 2^3
    let n = 2_usize.pow(3);
    
    // Calculate 2^n using u32
    let m = 2_u32.pow(n as u32); // 2^n

    // Generate binary vectors for n
    let inputs = generate_binary_vectors(n);

    // Write inputs and m to a CSV file
    let file_path = "binary_vectors.csv";
    write_vectors_and_m_to_csv(&inputs, m, file_path)?;
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

fn write_vectors_and_m_to_csv(vectors: &Vec<Vec<i32>>, m: u32, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    
    // Write the value of m at the top of the CSV
    writeln!(file, "2^(2^3) = {}", m)?;

    // Write a separator for clarity
    writeln!(file, "Binary vectors:")?;
    
    // Write the binary vectors in CSV format
    for vector in vectors {
        let line = vector.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        writeln!(file, "[{}]", line)?;
    }
    
    Ok(())
}
