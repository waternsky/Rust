/* Matrix multiplication implementation */
/* Matrix is represented as collection of it rows */
use std::io;

fn main() {

    let mut input = String::new();
    
    let input: u32 = loop {
        
        println!("Please type a natural number");

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read line");

        match input.trim().parse::<u32>() {
            Ok(num) => break num,
            Err(_) => {
                println!("Type a natural number!!");
                input = String::new();
                continue;
            }
        }
    };

    println!("User has typed: {input}");

}


