use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut dial: i64 = 50;
    let mut last_dial: i64;
    let mut zero_count: u64 = 0;
    println!("Beginning process...");
    let p = Path::new("input.txt");
    if !p.exists() {
        eprintln!("Could not find 'input.txt'");
        std::process::exit(1);
    }
    if let Ok(lines) = read_lines("input.txt") {
        println!("Reading lines from: 'input.txt'");
        for line in lines.map_while(Result::ok) {
            last_dial = dial;
            let mut delta: i64 = parse_delta(line); // delta can be +/-
            zero_count += delta.unsigned_abs() % 100; // for cases where |delta| > 99
            delta %= 100;
            dial += delta;
            let diff = (dial - last_dial).abs();
            let sum = diff + delta;
            if !(1..100).contains(&sum) {
                zero_count += 1;
                println!("({dial}, {delta}, {sum})");
            }
            dial %= 100;
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

/// Takes in a string starting with 'L' or 'R' and an unsigned integer, and returns a signed
/// integer
#[inline]
fn parse_delta(line: String) -> i64 {
    if let Some(stripped) = line.strip_prefix('L') {
        -stripped.parse::<i64>().unwrap().abs()
    } else if let Some(stripped) = line.strip_prefix('R') {
        stripped.parse::<i64>().unwrap().abs()
    } else {
        panic!("This should be unreachable.");
    }
}
