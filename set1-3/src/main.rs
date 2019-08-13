use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
// http://www.data-compression.com/english.html
const TABLE: [f64;27] = [0.0651738, 0.0124248, 0.0217339, 0.0349835, 0.1041442, 0.0197881, 0.0158610, 0.0492888, 0.0558094, 0.0009033, 0.0050529, 0.0331490, 0.0202124, 0.0564513, 0.0596302, 0.0137645, 0.0008606, 0.0497563, 0.0515760, 0.0729357, 0.0225134, 0.0082903, 0.0171272, 0.0013692, 0.0145984, 0.0007836, 0.1918182];

fn load_file(file: &str) -> Vec<String> {
	let file = File::open(file).unwrap();
	let buf = BufReader::new(file).lines();
	buf.map(|l| l.unwrap().to_ascii_lowercase()).collect()
}

// rates text based on word occurrence number
fn rate(text: &Vec<u8>, dict: &Vec<String>) -> f64 {
	let mut count = 0;

	for word in String::from_utf8(text.clone()).unwrap_or_else(|_| String::new()).split(' ') {
		if dict.contains(&word.to_string().to_ascii_lowercase()) {
			count += 1;
		}
	}

	return count as f64
}

// rates text based on letter occurrence number
fn rate_2(text: &Vec<u8>) -> f64 {
	return text.iter()
		.map(|&e| e as char)
		.map(|e| e.to_ascii_lowercase())
		.filter(|&e| e.is_ascii_alphabetic() || e == ' ')
		.map(|e| match e {
			' ' => TABLE[26],
			n => TABLE[n as usize - 97 as usize]
		})
		.sum();
}

fn main() {
	let lines = load_file("../words_alpha.txt");

	let mut best_pick =  ("".to_string(), 0_f64);

	for i in 0..255 {
		println!("attempt {} ({}/{})", i as char, i, 255);

		let mut data = hex::decode(INPUT).unwrap();
		data = data.iter().map(|&c| c ^ i).collect();

		//let r = rate(&data, &lines);
		let r = rate_2(&data);
		if r > best_pick.1 {
			let out = String::from_utf8(data).unwrap();
			println!("New best pick: {} ({})", &out, r);
			best_pick = (out, r);
		}
	}

	println!("Best pick: {:?}", best_pick);
}