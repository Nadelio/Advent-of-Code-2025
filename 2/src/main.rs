use fancy_regex::Regex;

fn main() {
	let input = std::fs::read_to_string("input.txt").expect("Failed to read from file: 'input.txt'");
	let rgx = Regex::new(r"^([1-9]\d*)\1$").unwrap();
	let ranges = parse_input(input);
	let mut password: u64 = 0;
	for range in ranges {
		for i in range {
			let result = rgx.is_match(&i.to_string());
			if let Ok(result) = result {
				let did_match = result;
				if did_match {
					println!("Matched {i} successfully");
					password += i;
				}
			}
		}
	}
	println!("Password: {password}");
}

fn parse_input(input: String) -> Vec<std::ops::RangeInclusive<u64>> {
	let lines = input.split(',');
	let mut ranges: Vec<std::ops::RangeInclusive<u64>> = vec![];
	for line in lines {
		println!("{line}");
		// get the first number
		let lhs = line.split('-').next().unwrap().parse::<u64>().unwrap();
		// get the second number
		let rhs = line
			.split('-')
			.nth(1)
			.unwrap()
			.trim()
			.parse::<u64>()
			.unwrap();
		// combine them to a range
		let range = lhs..=rhs;
		println!("{range:#?}");
		ranges.push(range);
	}
	ranges
}
