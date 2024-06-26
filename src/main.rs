// main.rs

mod model;
mod generate_data_set;

use generate_data_set::{DataCollection, generate_data_set};

use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let dataset_file_path = "data_sets.json";
    let n = 2; //number of bits 

    // Check if the dataset file already exists
    if !Path::new(dataset_file_path).exists(){
        println!("Dataset file not found, generating new dataset...");
        generate_data_set(n)?;
    } else {
        println!("Dataset file already exists, proceeding without generation.");
    }

    // Read the generated JSON file
    let data = fs::read_to_string(dataset_file_path).expect("Unable to read file");
    let collection: Result<DataCollection, serde_json::Error> = serde_json::from_str(&data);

    match collection {
        Ok(col) => {
            let datasets_to_process = col.data_sets.iter();

            for dataset in datasets_to_process {
                model::process_dataset(dataset);
            }

        },
        Err(e) => {
            println!("Error parsing JSON: {:?}", e);
        }
   }

    Ok(())
}
