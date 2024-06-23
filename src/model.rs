// model.rs

use crate::generate_data_set::DataSet;

pub fn process_dataset(dataset: &DataSet) {
    println!("Processing DataSet with mu = {}", dataset.mu);
    println!("Inputs: {:?}", dataset.inputs);
    println!("Output: {:?}", dataset.output);
}
