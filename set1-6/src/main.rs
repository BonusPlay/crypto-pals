use bit_vec::BitVec;
use std::fs;

// http://www.data-compression.com/english.html
const TABLE: [f64;27] = [0.0651738, 0.0124248, 0.0217339, 0.0349835, 0.1041442, 0.0197881, 0.0158610, 0.0492888, 0.0558094, 0.0009033, 0.0050529, 0.0331490, 0.0202124, 0.0564513, 0.0596302, 0.0137645, 0.0008606, 0.0497563, 0.0515760, 0.0729357, 0.0225134, 0.0082903, 0.0171272, 0.0013692, 0.0145984, 0.0007836, 0.1918182];

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

fn find_key_size(msg: &Vec<u8>) -> u32 {
	// key size, normalized hammering dist
	let mut best: (u32, f32) = (0, 999_f32);

	for i in 2..40 {
		let mut results: Vec<f32> = [].to_vec();

		for j in 0..4 {
			let p1 = &msg[i * (j * 2) .. i * (j * 2 + 1)];
			let p2 = &msg[i * (j * 2 + 1)..i * (j * 2 + 2)];

			results.push(get_hammering_dist(p1, p2) as f32);
		}

		let norm = results.iter().sum::<f32>() / (i * 4) as f32;

		if norm < best.1 || i == 2 {
			best = (i as u32, norm);
		}
	};

	return best.0;
}

// transposes blocks
fn step2(msg: &Vec<u8>, key_size: u32) -> Vec<Vec<u8>> {
	let mut parts: Vec<Vec<u8>> = [].to_vec();
	for i in 0..key_size {
		parts.push(msg.iter()
			.enumerate()
			.filter(|&(j, _)| j as u32 % key_size == i)
			.map(|(_, e)| *e)
			.collect());
	}
	return parts
}

fn step3(part: Vec<u8>) -> String {
	// key char, rating
	let mut best_pick = (' ', 0_f64, String::new());

	for j in 0..255 {
		let out = &part.iter().map(|&c| c ^ j).collect();

		let r = rate(out);
		if r > best_pick.1 {
			best_pick = (j as char, r, String::from_utf8(out.to_owned()).unwrap());
		}
	}

	println!("{:?}", best_pick);
	return best_pick.2
}

// rates text based on letter occurrence number
fn rate(text: &Vec<u8>) -> f64 {
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
	let input = fs::read_to_string("input.txt").unwrap();
	let msg = base64::decode(&input.replace("\n", "")).unwrap();

	let key_size = find_key_size(&msg);
	println!("key size: {:?}", key_size);

	let parts = step2(&msg, key_size);
	for part in &parts {
		println!("part: {:?}", part);
	}

	let mut data: Vec<Vec<u8>> = [].to_vec();
	for part in parts {
		data.push(step3(part).into_bytes());
	}

	// reorder blocks
	let mut fin: Vec<u8> = [].to_vec();

	for i in 0..data[0].len() {
		for j in 0..data.len() {
			if i* key_size as usize + key_size as usize > data[j].len() {
				fin.extend_from_slice(&data[j][i..]);
			} else {
				fin.extend_from_slice(&data[j][i * key_size as usize..(i + 1) * key_size as usize]);
			}
		}
	}

	println!("{:?}", String::from_utf8(fin).unwrap());
}

#[cfg(test)]
mod tests {
	#[test]
	fn hammering() {
		assert_eq!(37, super::get_hammering_dist("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()))
	}
}