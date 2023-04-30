use std::io;

fn main() {
    
    println!("How many fibonacci numbers do you want?");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read line!");
    
    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Please type natural number!"),
    };

    println!("You typed: {input}");

    let ans = fibonacci(input);

    println!("Array of fibonacci numbers: {:?}", ans);
}

fn fibonacci(n: u32) -> Vec<u128> {
   
    let num = n as usize;

    let mut v: Vec<u128> = Vec::new(); 

    v.push(1);
    v.push(1);

    for i in 2..=num {
        v.push(&v[i-1]+&v[i-2]);
    }

    v
}

