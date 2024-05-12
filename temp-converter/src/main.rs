use std::io;

fn main() {
    println!("What would you like to convert? Please enter 'f' for Fahrenheit to Celsius or 'c' for Celsius to Fahrenheit.");
    let mut conversion = String::new();

    io::stdin()
        .read_line(&mut conversion)
        .expect("Failed to read line");

    let conversion = conversion.trim();
    assert!(conversion == "f" || conversion == "c", "Please enter 'f' or 'c'.");

    println!("Please enter the temperature you would like to convert.");
    let mut temp = String::new();
    
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f64 = temp.trim().parse().expect("Please type a number!");

    if conversion == "f" {
        println!("{} Fahrenheit is {} Celsius.", temp, fahrenheit_to_celsius(temp));
    } else {
        println!("{} Celsius is {} Fahrenheit.", temp, celsius_to_fahrenheit(temp));
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
