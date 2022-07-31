use std::io;
use std::collections::HashMap;


fn main() {

	let canidate_list = set_canidate_list();
	let word = get_word();
	
	for canidate in canidate_list.iter() {
		if check_if_anagram_by_hashmap(&word, &canidate) {
			println!("{}", canidate);
		}
	}

}

fn get_word() -> String {
	
	println!("Enter original word");
	
	let mut input_string = String::new();
	
	io::stdin()
		.read_line(&mut input_string)
		.expect("Couldn't readline");
		
	input_string
}

fn set_canidate_list() -> Vec<String> {
	
	println!("Enter cans until quit");
	
	let mut list: Vec<String> = Vec::new();
	
	loop {
		
		let mut word = String::new();
		
		println!("Enter words to list or quit to quit listing 😂");
		
		io::stdin()				
			.read_line(&mut word)
			.expect("Couldn't read line, yikes.");
		
		if (word.trim()).eq("quit") {
			break;
		}
		
		list.push(word);	
	}
	
	println!("the words:");
	for word in &list {
		println!("{}", word);
	}
	
	list
}

fn check_if_anagram(main_word: &String, secondary_word: &String) -> bool {
	
	if main_word.eq(secondary_word) || main_word.len() != secondary_word.len() {
		return false
	}
	
	let mut sorted_main = main_word.clone().into_bytes();
	let mut sorted_scnd = secondary_word.clone().into_bytes();
	
	sorted_main.sort_unstable();
	sorted_scnd.sort_unstable();
	
	for (i, &ele) in sorted_main.iter().enumerate() {
		if ele != sorted_scnd[i] {
			return false
		}
	}
	
	true
}

fn check_if_anagram_by_hashmap(main_word: &String, secondary_word: &String) -> bool {
	
	if main_word.eq(secondary_word) || main_word.len() != secondary_word.len() {
		return false
	}
		
	let mut main_map = HashMap::new();
	let mut secondary_map = HashMap::new();

	for main_word_letter in main_word.chars() {
		let letter_count = main_map.entry(main_word_letter).or_insert(0);
		*letter_count += 1;
	}
	
	for secondary_word_letter in secondary_word.chars() {
		let letter_count = secondary_map.entry(secondary_word_letter).or_insert(0);
		*letter_count += 1;
	}
	
	//println!("main_word map: {:?}\nsecondary_word map:{:?}", main_map, secondary_map);
	
	for (key, value) in &main_map {		
		if !secondary_map.contains_key(&key) || value != secondary_map.get(&key).unwrap_or(&0) {
			return false
		}
	}
	
	true
}