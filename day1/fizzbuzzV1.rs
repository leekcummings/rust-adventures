fn main() {
    // Fizz buzz program
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz")
        } else if n % 3 == 0 {
            println!("fizz")
        } else if n % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", n)
        }
    }
}