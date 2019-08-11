const INPUT: &str = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
const KEY: &str = "ICE";
const OUTPUT: &str = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

fn main() {
	let mut out: Vec<u8> = [].to_vec();

	for (j, c) in INPUT.chars().enumerate() {
		let cur_key = KEY.chars().nth(j % KEY.len()).unwrap();
		out.push( c as u8 ^ cur_key as u8);
	}

	assert_eq!(out, hex::decode(OUTPUT).unwrap());
	println!("OK");
}