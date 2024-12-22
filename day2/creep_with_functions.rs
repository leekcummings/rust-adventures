use std::io;

fn get_age(count: u8) -> f32{
    let mut input = String::new();
    println!("Enter age {}:", count);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let age = match input.trim().parse().unwrap() {
        Ok(age) => age,
        Err(error) => panic!("{error:?}"),
    };
    7.0
}

fn main() {
    // Welcome to Creepy Dating!
    // This program tests if you are old enough to date that person
    // (Obligatory this is a joke)
    let mut count: u8 = 1;
    let age1 = get_age(count);
    count += 1;
    let age2 = get_age(count);
    
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