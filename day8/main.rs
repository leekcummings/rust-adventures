use std::io;

// My orignal code
// There's nothing wrong with it, I just wanted to try to not create a new String
fn original_reverse(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut reverse = String::new();
    for c in chars.into_iter().rev() {
        reverse.push(c);
    }
    reverse
}

// New code! It uses .clone to allow me to reference the str and the mut ref to write the new str
fn reverse(input: &mut String) -> String {
    input.push_str("\n"); // add a new line so I can split it later
    for c in input.clone().chars().rev() {
        input.push(c);
    }
    input.split("\n").collect::<Vec<&str>>()[2].to_string()
} 

fn main() {
    let mut input = String::new();
    println!("Write a string and I will reverse it: ");
    io::stdin().read_line(&mut input).expect("Could not read line");
    input = input.strip_suffix("\n").unwrap().to_string();
    println!("Original Reversal: {}", original_reverse(&input));
    println!("Shorter Reversal: {}", reverse(&mut input.to_string()));
}