use std::fs::File;
use std::io::{BufReader, BufRead};
use bit_vec::BitVec;

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

pub fn get_hammering_dist(a: &[u8], b: &[u8]) -> u32 {
	let mut counter = 0;

	let a_bytes = BitVec::from_bytes(a);
	let b_bytes = BitVec::from_bytes(b);

	for (i, c1) in BitVec::iter(&a_bytes).enumerate() {
		let c2 = BitVec::get(&b_bytes, i).unwrap();
		if c1 != c2 {
			counter += 1;
		}
	}

	return counter;
}

// finds key size
fn step1(msg: &String) -> u32 {
	// key size, normalized hammering dist
	let mut best: (u32, f32) = (0, 999_f32);

	for i in 2..40 {
		let mut results: Vec<f32> = [].to_vec();

		for _ in 0..3 {
			results.push(get_hammering_dist(msg[0..i].as_bytes(), msg[i..i + i + 1].as_bytes()) as f32 / i as f32);
		}

		let norm = results.iter().sum::<f32>() / (results.len() as f32);

		if norm < best.1 || i == 2 {
			best = (i as u32, norm);
		}
	};

	return best.0;
}

// transposes blocks
fn step2(msg: &String, key_size: u32) -> Vec<Vec<u8>> {
	let mut parts: Vec<Vec<u8>> = [].to_vec();
	for i in 0..key_size {
		parts.push(msg.as_bytes()
			.iter()
			.enumerate()
			.filter(|&(j, _)| (j as u32 + i) % key_size == 0)
			.map(|(_, e)| *e)
			.collect());
	}
	return parts
}

fn step3(part: Vec<u8>) -> String {
	// key char, rating
	let mut best_pick = (' ', 0, String::new());

	for j in ' ' as u8..'~' as u8 {
		let out = &part.iter().map(|&c| c ^ j).collect();

		let r = rate(out);
		if r > best_pick.1 {
			println!("New best pick: {:?} ({})", out, r);
			best_pick = (j as char, r, String::from_utf8(out.to_owned()).unwrap());
		}
	}

	return best_pick.2
}

// rates text based on letter occurrence number
fn rate(text: &Vec<u8>) -> u32 {
	return text.iter()
		.map(|&e| e as char)
		.map(|e| e.to_ascii_lowercase())
		.filter(|&e| "ETAOINSHRDLU".to_ascii_lowercase().contains(e))
		.count() as u32;
}

fn main() {
	let input = load_file("input.txt");
	let msg = input.join("");

	let key_size = step1(&msg);
	println!("key size: {:?}", key_size);

	let parts = step2(&msg, key_size);
	println!("parts: {:?}", parts);

	let mut data: Vec<Vec<u8>> = [].to_vec();
	for part in parts {
		data.push(step3(part).into_bytes());
	}

	// reorder blocks
	let mut fin: Vec<u8> = [].to_vec();
	let mut iter = data[0].iter();
	while let Some(x) = iter.next() {
		match x {
			x => println!("{:?}", x)
		}
	}
//	while let Some(x) = data[0].iter().next() {
//		println!("{:?}", x);
//	}
//	println!("{:?}", String::from_utf8(data));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn hammering() {
		assert_eq!(37, super::get_hammering_dist("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()))
	}
}