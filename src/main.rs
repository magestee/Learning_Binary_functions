// main.rs

mod generate_data_set;
mod model;

use generate_data_set::{DataCollection, generate_data_set};
use serde_json::{self, Error};
use std::fs;

fn main() -> std::io::Result<()> {
    let n = 4; // Adjust `n` here as needed
    generate_data_set(n)?; // Ensure data is generated

    // Now read the generated JSON file
    let data = fs::read_to_string("data_sets.json").expect("Unable to read file");
    let collection: Result<DataCollection, Error> = serde_json::from_str(&data);

    match collection {
        Ok(col) => {
            // Find the dataset with mu = 1
            if let Some(dataset) = col.data_sets.iter().find(|d| d.mu == 256) {
                model::process_dataset(dataset); // Process this dataset in model.rs
            } else {
                println!("No dataset with mu = 1 found.");
            }
        },
        Err(e) => {
            println!("Error parsing JSON: {:?}", e);
        }
    }

    Ok(())
}
