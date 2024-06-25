// main.rs
mod generate_data_set;
mod model;

use generate_data_set::{DataCollection, generate_data_set};
use serde_json::{self, json};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let dataset_file_path = "data_sets.json";
    let accuracies_file_path = "accuracies.json";
    let n = 4; //number of bits 

    // Check if the dataset file already exists
    if !Path::new(dataset_file_path).exists(){
        println!("Dataset file not found, generating new dataset...");
        generate_data_set(n)?;  // Generate the dataset if it does not exist
    } else {
        println!("Dataset file already exists, proceeding without generation.");
    }

    // Read the generated JSON file
    let data = fs::read_to_string(dataset_file_path).expect("Unable to read file");
    let collection: Result<DataCollection, serde_json::Error> = serde_json::from_str(&data);

    match collection {
        Ok(col) => {
            let mut accuracies = Vec::new();
            let datasets_to_process = col.data_sets.iter();

            for dataset in datasets_to_process {
                let accuracy = model::process_dataset(dataset);
                accuracies.push(accuracy);
            }

            if !accuracies.is_empty() {
                // Serialize and save the accuracies to a JSON file
                let json_acc = json!(accuracies);
                let mut file = File::create(accuracies_file_path)?;
                writeln!(file, "{}", json_acc)?;
                println!("Accuracies written to file successfully.");

                // Read the accuracies file and calculate the average
                let accuracies_data = fs::read_to_string(accuracies_file_path)
                    .expect("Unable to read accuracies file");

                let accuracies_json: Vec<f64> = serde_json::from_str(&accuracies_data)
                    .expect("Error parsing accuracies JSON");

                let average_accuracy = accuracies_json.iter().sum::<f64>() / accuracies_json.len() as f64;
                println!("Average Accuracy: {:.2}%", average_accuracy);

            } else {
                println!("No datasets found, nothing written.");
            }
        },
        Err(e) => {
            println!("Error parsing JSON: {:?}", e);
        }
   }

    Ok(())
}
