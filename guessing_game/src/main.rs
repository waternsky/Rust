use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut counter = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                counter = counter + 1;
            },
            Ordering::Greater => {
                println!("Too big!");
                counter = counter + 1;
            },
            Ordering::Equal => {
                println!("You win!");
                println!("Number of attempts: {counter}");
                break;
            }
        }
    }
    

}
