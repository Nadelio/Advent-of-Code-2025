use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut zero_count = 0;
    println!("Beginning process...");
    let p = Path::new("input.txt");
    if !p.exists() {
        eprintln!("Could not find 'input.txt'");
        std::process::exit(1);
    }
    if let Ok(lines) = read_lines("input.txt") {
        println!("Reading lines from: 'input.txt'");
        for line in lines.map_while(Result::ok) {
            zero_count += part_two(&line).unwrap_or_default();
        }
    }
    println!("Number of Zeros: {zero_count}");
}

/// Reads in all the lines of a file and returns a Lines struct
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
