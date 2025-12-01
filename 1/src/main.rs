use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut dial = 50;
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
            println!("{line}");
            let delta = process(line);
            dial += delta;
            dial %= 100;
            if dial == 0 {
                zero_count += 1;
            }
        }
    }
    println!("Number of Zeros: {zero_count}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[inline]
fn process(line: String) -> i32 {
    if let Some(stripped) = line.strip_prefix('L') {
        println!("Rotate left {stripped} places");
        -stripped.parse::<i32>().unwrap().abs()
    } else if let Some(stripped) = line.strip_prefix('R') {
        println!("Rotate right {stripped} places");
        stripped.parse::<i32>().unwrap().abs()
    } else {
        panic!("This should be unreachable.");
    }
}
