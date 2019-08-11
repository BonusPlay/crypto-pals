use std::fs::File;
use std::io::{BufRead, BufReader};

// https://stackoverflow.com/a/49806368
macro_rules! skip_fail {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                continue;
            }
        }
    };
}

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
	let dict = load_file("../words_alpha.txt");
	let input = load_file("input.txt");

	let mut best_pick =  ("".to_string(), 0);

	for (i, line) in input.iter().enumerate() {
		println!("attempt ({}/{})", i, input.len());
		for j in ' ' as u8 .. '~' as u8 {
			let mut data = skip_fail!(hex::decode(line));
			data = data.iter().map(|c| *c ^ j).collect();
			let out = skip_fail!(String::from_utf8(data));

			let r = rate(&out, &dict);
			if r > best_pick.1 {
				println!("New best pick: {} ({})", out, r);
				best_pick = (out, r);
			}
		}
	}

	println!("Best pick: {:?}", best_pick);
}