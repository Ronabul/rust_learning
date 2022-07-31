use std::io;

fn main() {
    
	println!("Enter the number of the element in fibo");
	
	let mut wanted_ele = String::new();
	
	io::stdin()
		.read_line(&mut wanted_ele)
		.expect("Can't read");
	
	let mut wanted_ele: usize = match wanted_ele.trim().parse() {
		Ok(wanted_ele) => wanted_ele,
		Err(wanted_ele) => 10,
	};
		
	let number = fibonacci(wanted_ele);
	
	println!("Number: {number}");
	
}

fn fibonacci(element: usize) -> usize {
	if element < 2 {
		return element;
	}
	fibonacci(element - 1) + fibonacci(element - 2)
}