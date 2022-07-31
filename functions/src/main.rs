fn main() {
    println!("Hello, world!");
	another_func(5)
}

fn another_func(x: i32) {
	println!("The value of x is {x}");
}

fn five() -> i32 {
    5
}
