fn main() {
    // Fizzbuzz with concatenating strings instead
    for n in 1..=100 {
        let mut print = "".to_string();
        if n % 3 == 0 {
            print.push_str("fizz");
        }
        if n % 5 == 0 {
            print.push_str("buzz");
        }
        if print == "" {
            print = n.to_string();
        }
        println!("{}", print)
    };
}