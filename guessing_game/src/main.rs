use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Enter a number:");
    let sec_num = rand::thread_rng().gen_range(1..=50);
    let mut guess = String::new();
    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to load number");
        let guess: u32 = guess.trim().parse().expect("Please type a number!!!");
        println!("You guessed {guess}");
        match guess.cmp(&sec_num) {
            Ordering::Less => println!("number guessed is too low!"),
            Ordering::Greater => println!("number guessed is too big!"),
            Ordering::Equal => {
                println!("You guessed the correct number!!!");
                break;
            }
        }
    }
}
