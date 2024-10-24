use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    ExitCode::SUCCESS
}
