use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number???");
    //First, we generate a secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        //Now get the user input
        println!("Please input a number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //The line below thas the same parsing as the next line
        //let guess = guess.trim().parse::<u32>().expect("Error parsing guess");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //It seems that {} is used for string formatting
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
