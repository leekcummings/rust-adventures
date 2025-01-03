use std::io;

fn get_input() -> String {
    let mut input = String::new();
    println!("Give me a positive integer: ");
    io::stdin().read_line(&mut input).expect("Could not read line");
    input.strip_suffix("\n").expect("Why do I need this").to_string()
}

fn factorize(num: i32) -> Vec<i32> {
    let mut factors: Vec<i32> = Vec::new();
    let mut remaining = num;
    let mut fac = 2;
    while remaining != 1 {
        if remaining % fac == 0 {
            remaining /= fac;
            factors.push(fac);
        } else {
            fac += 1
        }
    }
    factors
}

fn main() {
    let input = loop {
        let input = get_input();
        if input.parse::<i32>().is_ok() && input.parse::<i32>().unwrap() > 1 {
            break input.parse::<i32>()
        }
    };
    println!("{:?}", factorize(input.expect("What")))
}