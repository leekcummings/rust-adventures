fn main() {
    // Make a list of primes using mut Vec
    let mut list: Vec<usize> = Vec::new();
    // Start by adding the first prime: 2
    list.push(2);

    for n in 3..=100 {
        let mut prime: bool = true;
        for p in &mut list {
            // The * deferences the item
            // Not sure why it works, but it works
            if n % *p == 0 {
                prime = false;
            }
        };

        // Add to Vec if prime
        if prime {
            list.push(n);
        }
    }

    print!("The prime numbers from 1-100 are: ");
    for p in &list {
        // Dereferencing again babyyyyyy
        if *p != list[list.len() - 1] {
            print!("{p:?}, ");
        } else {
            // Yay! Nice formatting! Good job!
            println!("{p:?}!");
        }
    }
}