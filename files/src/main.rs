use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

fn main() -> Result<(), Error> {
    // Path file creation and lines to iterate over it
    let path = "lines.txt";
    let lines = 1000;

    let mut output = File::create(path)?;
    // Write lines to file
    for _ in 0..lines {
        write!(output, "I💖 Rust\n")?;
    }

    // Open file and read lines
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    // Print results
    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}
