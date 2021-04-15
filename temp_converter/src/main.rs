use std::io;

fn main() {
    println!("Temperature converter.");

    loop {
        println!("Choose convertation type.");
        println!("Type 1 for: Celsius -> Fahrenheit");
        println!("Type 2 for: Fahrenheit -> Celsius");
        println!("Type q for quit");
    
        let mut convertation_type = String::new();
    
        io::stdin()
            .read_line(&mut convertation_type)
            .expect("Invalid input!");
        
        let convertation_type = convertation_type.trim();
    
        if convertation_type == "1" {
            convert_from_celsius();
            break;
        } else if convertation_type == "2" {
            convert_from_fahrenheit();
            break;
        } else if convertation_type == "q" {
            break;
        } else {
            eprintln!("Error: Unexpected convertation type!");
            continue;
        }
    }
}

fn convert_from_celsius() {
    let res = loop {
        println!("Type temperature in Celsius:");
        let mut value = String::new();
        
        io::stdin()
            .read_line(&mut value)
            .expect("Invalid input!");
    
        let value: f32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break (9.0 / 5.0) * value + 32.0
    };

    println!("Temperature in Fahrenheit is {}", res);
}

fn convert_from_fahrenheit() {
    let res = loop {
        println!("Type temperature in Fahrenheit:");
        let mut value = String::new();
        
        io::stdin()
            .read_line(&mut value)
            .expect("Invalid input!");
    
        let value: f32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break (5.0 / 9.0) * (value - 32.0)
    };

    println!("Temperature in Celsius is {}", res);
}