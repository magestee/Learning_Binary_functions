mod generate_data_set;

fn main() -> std::io::Result<()> {
    let n = 3; // Adjust `n` here as needed
    generate_data_set::generate_data_set(n)
}
