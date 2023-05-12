use std::fs::File;
use std::io::{Read,self};

fn main() {

    let name = read_username_from_file();

    println!("Name is {}", name.unwrap());
}

fn read_username_from_file() -> Result<String, io::Error> {

    let mut username_file =File::open("hello.txt")?;

    let mut username = String::new();

    username_file
        .read_to_string(&mut username)
        .expect("Unable to read name from file");

    Ok(username)
}
