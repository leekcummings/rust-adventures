use std::io;

fn get_input(turn: usize) -> String {
    println!("Enter Player {}'s move...", turn+1);
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Could not read line");
    input
}

fn main() {
    // two player rock, paper, scissors
    // this won't be too bad... right?
    let mut choices: [String; 2] = [String::new(), String::new()];

    let r = "rock".to_string();
    let p = "paper".to_string();
    let s = "scissors".to_string();
    let opts: [&str; 3] = [&r, &p, &s];
    
    for player in 0..2 {
        let mut valid_choice = false;
        while !valid_choice {
            // I don't understand ownership
            // I know no god except the rust compiler
            let choice = get_input(player).strip_suffix("\n").unwrap().to_owned();
            
            for o in &opts {
                if *o == choice {
                    valid_choice = true;
                }
            }
            choices[player] = choice;
        }
    }
    // I did not realize rust had switch cases before this
    // observe my beautiful if else tower instead
    if choices.contains(&r) && choices.contains(&s) {
        let index = choices.iter().position(|x| *x == r).unwrap();
        println!("Player {} wins!", index+1)
    } else if choices.contains(&r) && choices.contains(&p) {
        let index = choices.iter().position(|x| *x == p).unwrap();
        println!("Player {} wins!", index+1)
    } else if choices.contains(&p) && choices.contains(&s) {
        let index = choices.iter().position(|x| *x == s).unwrap();
        println!("Player {} wins!", index+1)
    } else {
        println!("It's a tie...")
    }
    // this was harder than I thought
}