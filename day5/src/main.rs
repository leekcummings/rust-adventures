use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn main() -> std::io::Result<()> {
    println!("There's been a murder...");
    println!("We have a list of possible suspects in a .csv");
    println!("Using the powers of regex, we can solve the murder");

    let mut file = File::open("all_data.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let people: std::str::Split<'_, &str> = contents.split("\n");
    
    println!("First, we know they work for a .org");

    let mut match1: Vec<&str> = Vec::new();
    let re: Regex = Regex::new(r".*\.org*").unwrap();
    for person in people {
        let mat: Vec<&str> = re.find_iter(person).map(|m| m.as_str()).collect();
        if !mat.is_empty() {
            &match1.push(person);
        } 
    }
    
    println!("The length of suspects with .org emails is: {}", match1.len());
    Ok(())
}