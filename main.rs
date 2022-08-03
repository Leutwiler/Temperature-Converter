use std::io;

fn main() {
    println!("Welcome to the temperature converter!");

    println!("Please, type F if you want to convert from Fahrenheit to Celsius or type C if you want to convert from Celsius to Fahrenheit");

    let scale = loop {
        let mut scale = String::new();

        io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read line");

        let scale: String = match scale.trim().to_uppercase().parse() {
            Ok(element) => element,
            Err(_) => continue,
        };

        if scale == "F" {
            break "F"
        } else if scale == "C" {
            break "C"
        } else {
            println!("Please, input F for Fahrenheit or C for Celsius");
        }
    };

    println!("Insert temperature");

    let temperature:f64 = loop {
        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        match temperature.trim().parse() {
            Ok(temperature) => break temperature,
            Err(_) => println!("Please, insert a valid temperature"),
        };
    };

    if scale == "F" {
        let result = 5 as f64 / 9 as f64 *(temperature - 32 as f64);
        println!("{result}°C");
    } else {
        let result = 9 as f64 / 5 as f64 * temperature + 32 as f64;
        println!("{result}°F")
    }
}
