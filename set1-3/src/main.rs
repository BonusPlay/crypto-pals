use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

fn load_file(file: &str) -> Vec<String> {
	let file = File::open(file).unwrap();
	let buf = BufReader::new(file).lines();
	buf.map(|l| l.unwrap().to_ascii_lowercase()).collect()
}

// rates text based on word occurrence number
fn rate(text: &String, dict: &Vec<String>) -> u32 {
	let mut count = 0;

	for word in text.split(' ') {
		if dict.contains(&word.to_string().to_ascii_lowercase()) {
			count += 1;
		}
	}

	return count
}

fn main() {
	let lines = load_file("../words_alpha.txt");

	let mut best_pick =  ("".to_string(), 0);

	for i in ' ' as u8 .. '~' as u8 {
		println!("attempt {} ({}/{})", i as char, i - ' ' as u8, '~' as u8 - ' ' as u8 - 1);

		let mut data = hex::decode(INPUT).unwrap();
		data = data.iter().map(|c| *c ^ i).collect();
		let out = String::from_utf8(data).unwrap();

		let r = rate(&out, &lines);
		if r > best_pick.1 {
			println!("New best pick: {} ({})", out, r);
			best_pick = (out, r);
		}
	}

	println!("Best pick: {:?}", best_pick);
}