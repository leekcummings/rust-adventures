use std::io;

fn create_array(primes: &[String], LEN: &usize, new: String) {
    for i in primes {
        print!("{}, ", i);
    }
    let mut items = String::new();
    io::stdin().read_line(&mut items).expect("Could not read line");
    items += &new;
    let p: [String] = [new];
    // println!("{p:?}");
}

fn main() {
    let blank: [String; 1] = ["2".to_string()];
    const LEN: usize = blank.len();
    create_array(&blank, &LEN, "3".to_string());
    // println!("{primes:?}");
}