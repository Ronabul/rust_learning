fn main() {
	
	//let hypedog = HousePets::Dog(Breed::Hound);
	let hypedog = HousePets::Dog(Breed::DogShit);
	let hypecat = HousePets::Cat;
	
	let newint: usize = match hypedog {
		HousePets::Cat => 3,
		HousePets::Pigglin => 900,
		HousePets::Dog(Breed::DogShit) => 1000000,
		HousePets::Dog(Breed::Bulldog) => 1,
		_ => panic!("wtffffffff"),
	} + hypecat.turn_pet_to_ten();
	
	println!("{}", newint);
}

enum Breed {
	Pomeranian,
	GoldenRetriver,
	Bulldog,
	Hound,
	DogShit,
}

enum HousePets {
	Cat,
	Dog(Breed),
	Pigglin,
	Fish,
	Lizard,
}

impl HousePets {
	fn turn_pet_to_ten(&self) -> usize {
		10
	}
}