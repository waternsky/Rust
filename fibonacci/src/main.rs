use std::io;

fn main() {
    println!("Nth fibonacci number calculator");
    
    let mut input = String::new();

    let input: usize = loop {
        println!("Please type valid number: ");
        
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read line");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    println!("User input: {input}");

    let ans = fibonacci(input);
    println!("Num: {ans}");
}

fn fibonacci(x: usize) -> u128 {
    
    let mut v = Vec::new();
    v.push(1);
    v.push(1);

    for i in 2..=x {
        v.push(v[i - 1] + v[i - 2]);
    }

    return v[x];
}
