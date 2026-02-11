use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("the secrete number is {secret_number}");
    loop {
        println!("Enter your number!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("the  number you guessed is {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too smol"),
            Ordering::Greater => println!("greater niga"),
            Ordering::Equal => {
                println!("You win! ");
                break;
            }
        }
    }
}
