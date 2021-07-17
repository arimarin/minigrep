use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Args to variables
    // let binary = &args[0]; // Not needed
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Error reading the file");

    println!("With text:\n{}", contents);
}
