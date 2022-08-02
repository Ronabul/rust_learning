use std::io;
use std::str;

fn main() {

	println!("EnterStr: ");
	
	let mut inp = String::new();	
	io::stdin()
		.read_line(&mut inp)
		.expect("can't read line");
	
	println!("EnterInt: ");
	
	let mut amp_number = String::new();
	io::stdin()
		.read_line(&mut amp_number)
		.expect("can't read line");

	let amp_number: u8 = amp_number.trim().parse().expect("Please Type num");

	let vec: Vec<u8> = ascii_up_by_param(&inp, amp_number);
	
	let new_str = match str::from_utf8(&vec) {
		Ok(ascii) => ascii,
		Err(ascii) => panic!("unicode. not 8 bit char")
	};
	
	println!("{}", new_str);

}

fn ascii_up_by_param(encoded_msg: &String, amp: u8) -> Vec<u8> {
	
	let mut byte_vec: Vec<u8> = vec![];
	let mut ascii: u8; 
	
	for byte in encoded_msg
					.as_bytes()
					.iter() {	
		ascii = *byte;
		if *byte > 96 && *byte < 123 {
			ascii = (ascii - 97 + amp)%26 + 97;
		}
		
		byte_vec.push(ascii);
	}
	
	byte_vec
}
