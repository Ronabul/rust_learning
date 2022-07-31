use std::io;

fn main() {
	let mut tempur = String::new();
	
	io::stdin()
		.read_line(&mut tempur)
		.expect("Couldn't read line");
		
	let tempur: isize = match tempur.trim().parse() {
		Ok(num) => num,
		Err(tempur) => 999999999,
	};
	
	println!("For Cels to Ferhn: {}", converter(tempur));
	
}

fn converter(celsius: isize) -> isize{
	celsius * 9/5 + 32
}