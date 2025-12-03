use std::fs::File;
use std::io::{self, prelude::*};
use std::path::PathBuf;
use std::str::FromStr;

// Enum to ensure that only Left and Right are possible values
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct ParseDirectionError;

// parse L and R to direction enums
impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(ParseDirectionError),
        }
    }
}

fn main() -> std::io::Result<()> {
    let file_contents = file_contents_to_string("input.txt");
    let contents = match file_contents {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    };

    let mut dial_ends_at_zero = 0;
    let mut dial_passes_zero = 0;
    let mut dial_position = 50;
    for line in contents.lines() {
        let (input_dir, num) = line.split_at(1); // split after first char
        let num = num.parse::<i32>().unwrap_or(0);
        let direction = input_dir.parse::<Direction>().unwrap();

        let zero_passes;
        (dial_position, zero_passes) = turn_dial(dial_position, direction, num);
        dial_passes_zero += zero_passes;
        if dial_position == 0 {
            dial_ends_at_zero += 1;
        }
    }
    dial_passes_zero += dial_ends_at_zero;

    println!("Original Password: {dial_ends_at_zero}");
    println!("Password method 0x434C49434B: {dial_passes_zero}");
    Ok(())
}

fn file_contents_to_string(path_to_file: &str) -> Result<String, io::Error> {
    // Path to Cargo.toml directory (crate root)
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    // File path relative to project root
    let file_path = PathBuf::from(manifest_dir).join(path_to_file);

    let mut file = File::open(&file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn turn_dial(start_pos: i32, direction: Direction, amount: i32) -> (i32, i32) {
    let mut curr_pos = start_pos;
    let mut times_passes_zero = 0;
    match direction {
        Direction::Left => {
            curr_pos -= amount;
            times_passes_zero = (curr_pos / 100).abs(); //integer division
            if curr_pos < 0 {
                // zero passes are double counted without this check
                if start_pos != 0 {
                    times_passes_zero += 1;
                }
                if (100 + curr_pos) % 100 == 0 {
                    times_passes_zero -= 1;
                }
                curr_pos = 100 - curr_pos.abs();
            }
            (curr_pos % 100, times_passes_zero)
        }
        Direction::Right => {
            curr_pos += amount;
            if curr_pos >= 100 {
                times_passes_zero = curr_pos / 100; //integer division
                if curr_pos % 100 == 0 || start_pos == 0 {
                    times_passes_zero -= 1;
                }
            }
            (curr_pos % 100, times_passes_zero)
        }
    }
}
