#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Dial {
    size: usize,
    position: usize,
    zeroes: usize,
}

impl Dial {
    fn new(size: usize, position: usize) -> Self {
        Dial {
            size,
            position,
            zeroes: 0,
        }
    }

    fn apply(&self, turn: Turn) -> Self {
        let mut new_dial = *self;

        // Turn the dial, wrapping (in both directions) as needed
        match turn.direction {
            Direction::Left => {
                new_dial.position = (new_dial.position + new_dial.size
                    - (turn.steps % new_dial.size))
                    % new_dial.size;
            }
            Direction::Right => {
                new_dial.position =
                    (new_dial.position + (turn.steps % new_dial.size)) % new_dial.size;
            }
        }

        // Record if we landed on zero
        if new_dial.position == 0 {
            new_dial.zeroes += 1;
        }
        new_dial
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Turn {
    direction: Direction,
    steps: usize,
}

impl From<&str> for Turn {
    fn from(s: &str) -> Self {
        let (dir_char, steps_str) = s.split_at(1);
        let direction = match dir_char {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction character"),
        };
        let steps: usize = steps_str
            .parse()
            .expect(&format!("Invalid number of steps in {s}"));
        Turn { direction, steps }
    }
}

fn part1(input: &str) -> impl Into<String> {
    input
        .lines()
        .map(Turn::from)
        .fold(Dial::new(100, 50), |dial, turn| dial.apply(turn))
        .zeroes
        .to_string()
}

fn part2(input: &str) -> impl Into<String> {
    input
        .lines()
        .map(Turn::from)
        // Replace each turn into multiple turns of 1 step each
        // It would be faster to calculate how many times we pass directly...
        // But that's *really* not necessary at this scale
        .flat_map(|turn| {
            std::iter::repeat(Turn {
                direction: turn.direction,
                steps: 1,
            })
            .take(turn.steps)
        })
        .fold(Dial::new(100, 50), |dial, turn| dial.apply(turn))
        .zeroes
        .to_string()
}

fn main() {
    // Read input from input.txt file
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt file");
    
    let result = part2(&input);
    println!("Part 2 result: {}", result.into());
}