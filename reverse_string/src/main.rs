use std::io;
//Lib used for parsing and making unicode strings comfortable to use
use unicode_segmentation::UnicodeSegmentation;

fn main() {

    let string_input = "😪😶😐😉🤣";

/*
    let mut string_input = new::String();

    std::stdin()
        .read_line(&mut string_input)
        .expect("Couldn't Read Line");
*/
    let mut reversed = String::new();
	
    reversed = reverse_with_unicode(string_input);
}

fn reverse_with_unicode(input: &str) -> String {
	
	let mut reversed_string = String::new();
	
	
	for letter in  input
					.graphemes(true)
					.rev() {
						
		reversed_string += letter;
	}
	
	println!("{}", reversed_string);
	
	reversed_string
}
