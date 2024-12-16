use std::io;

fn main() {
    // Welcome to Creepy Dating!
    // This program tests if you are old enough to date that person
    // (Obligatory this is a joke)
    let mut input = String::new();
    println!("Enter age 1:");
    io::stdin().read_line(&mut input).expect("Could not read line");
    let age1: f32 = input.trim().parse().unwrap();

    // CLEAR THE STRING CLEAR IT CLEAR CLEAR
    input = String::new();
    println!("Enter age 2:");
    io::stdin().read_line(&mut input).expect("Could not read line");
    let age2: f32 = input.trim().parse().unwrap();

    // Lower threshold
    if (age1 / 2.0) + 7.0 > age2 {        
        println!("Creep");
    // Upper threshold
    } else if (age1 - 7.0) * 2.0 < age2 {
        println!("That's creepy...");
    // Chill threshold
    } else {
        println!("Not creepy")
    };
}