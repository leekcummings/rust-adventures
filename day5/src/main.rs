use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

// This took me way longer than I thought it would
// Turns out learning file handling and regex at the same time is difficult
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
    
    println!("The number of suspects with .org emails is: {}", match1.len());
    println!("Next we know this person's first name is between 3 and 7 letters long");

    // Start of day 6
    // I tried to turn this into a function and it went VERY BADLY
    // I just want to be done with this project...
    let mut match2: Vec<&str> = Vec::new();
    let re: Regex = Regex::new(r"^.{3,7},").unwrap();
    for person in match1 {
        let mat: Vec<&str> = re.find_iter(person).map(|m| m.as_str()).collect();
        if !mat.is_empty() {
            &match2.push(person);
        } 
    }

    println!("The number of remaining suspects with a 3-7 letter name: {}", match2.len());
    println!("The suspect was born in May, June, July, or August");

    let mut match3: Vec<&str> = Vec::new();
    let re: Regex = Regex::new(r"[5678]/\d{2}/\d{4}").unwrap();
    for person in match2 {
        let mat: Vec<&str> = re.find_iter(person).map(|m| m.as_str()).collect();
        if !mat.is_empty() {
            &match3.push(person);
        } 
    }

    println!("The number of remaining suspects with a May-August birthday: {}", match3.len());
    println!("Finally, we know the last 4 digits of the suspect start and end with the same number");
    println!("Their first and last names also start with the same letter");

    let mut match4: Vec<&str> = Vec::new();
    // So APPARENTLY rust doesn't allow for regex backtracing
    // It's okay, we can work with this
    let re: Regex = Regex::new(r"([A-Z]).*,([A-Z]).*,.*,.*(\d)\d{2}(\d)").unwrap();
    for person in match3 {
        let mat= re.captures(person).unwrap();
        if &mat[1] == &mat[2] && &mat[3] == &mat[4]{
            &match4.push(person);
        }
    }

    println!("The number of remaining suspects with a May-August birthday: {}", match4.len());
    println!("With our suspect found, I will print their info to a file");

    // Added file writing for extra challenge
    // It was challenging
    let mut file = File::create("murderer.txt")?;
    file.write_all(match4[0].as_bytes())?;

    Ok(())
}