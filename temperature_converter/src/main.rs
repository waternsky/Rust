/*
 * Before you begin running, following is needed:
 *   Need to make sure that you can type º charcter, utf-8 charcters (special character).
 *
 */


use std::io;

enum Temperature {
    Farhenheit(f32),
    Celsius(f32)
}


fn main() {

    let mut temperature = String::new();

    println!("Input the temperature");

    let temperature: Option<Temperature> = loop {
        
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read the temperature");

        match temperature.trim() {
            inp => {
                // º is 2 bytes character hence 3 instead of 2 below!!
                let (num, unit) = inp.split_at(inp.len() - 3);
            
                let num: f32 = num.parse().expect("Unable to read valid number");

                match unit {
                    "ºF" => break Some(Temperature::Farhenheit(num)),
                    "ºC" => break Some(Temperature::Celsius(num)),
                    _ => {
                        println!("Please input temperature in correct format. e.g, 32ºC");
                        temperature = String::new();
                        continue;
                    }
                }
            }
        }
    };

    let temperature: Temperature = match temperature {
        Some(temp) => convert(temp),
        None => {
            println!("Invalid temperature");
            panic!("Temperature unknown!!")
        }
    };

    
    match temperature {
        Temperature::Farhenheit(num) => println!("Converted temperature: {:.3}ºF", num),
        Temperature::Celsius(num) => println!("Converted temperature: {:.3}ºC",num)
    }
}

fn convert(temperature: Temperature) -> Temperature {
    match temperature {
        Temperature::Farhenheit(num) => {
            println!("Temperature: {num}ºF is converted to celsius");
            Temperature::Celsius((num - 32.0) * (5.0/9.0))
        },
        Temperature::Celsius(num) => {
            println!("Temperature: {num}ºC is converted to Farhenheit");
            Temperature::Farhenheit((9.0/5.0) * num + 32.0)
        }
    }
}
