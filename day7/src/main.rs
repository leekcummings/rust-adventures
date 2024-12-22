use rand::{seq::SliceRandom, Rng};

struct Card {
    suit: char,
    rank: String,
    value: i32,
    is_ace: bool,
}

fn main() {
    // Create list of cards
    let mut deck: Vec<Card> = Vec::new();
    let suits: [char; 4] = ['♣', '♦', '♥', '♠'];
    let faces: [&str; 3] = ["J", "Q", "K"];
    for s in suits {
        // Aces
        let card = Card {
            suit: s,
            rank: "A".to_string(),
            value: 1,
            is_ace: true,
        };
        deck.push(card);
        // Number cards
        for num in 2..=10 {
            let card = Card {
                suit: s,
                rank: num.to_string(),
                value: num,
                is_ace: false,
            };
            deck.push(card);
        };
        // Face cards
        for f in faces {
            let card = Card {
                suit: s,
                rank: f.to_string(),
                value: 10,
                is_ace: false,
            };
            deck.push(card);
        }
    }
    // Shuffle deck and pick first 3 cards to total
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    println!("You are given 3 cards...");
    let mut score = 0;

    for i in 0..3 {
        score += deck[i].value;
        let mut s = deck[i].value;
        if deck[i].is_ace {
            s = 11;
        }
        println!("{} of {} (Score of {})", deck[i].rank, deck[i].suit, s);
    }
    println!("You got a total score of: {}", score)
}
