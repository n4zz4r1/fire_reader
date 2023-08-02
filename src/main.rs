use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn read_file(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_path = &args[1];
    match read_file(file_path) {
        Ok(_) => println!("File read successfully."),
        Err(e) => eprintln!("Error while reading the file: {}", e),
    }
}
