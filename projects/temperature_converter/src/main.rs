use std::io;

fn main() {

    println!("Please enter a temperature: ");

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    
    let temp: i16 = buffer
        .trim()
        .parse()
        .expect("Wanted a number");

    let fahr_to_cel = fahrenheit_to_celsius(temp);
    println!("From Fahrenheit to Celsius: {fahr_to_cel}");

    let cel_to_fahr = celsius_to_fahrenheit(temp);
    println!("From Celsius to Fahrenheit: {cel_to_fahr}");

    let fahr_to_kelvin = fahrenheit_to_kelvin(temp.into());
    println!("From Fahrenheit to Kelvin: {fahr_to_kelvin}");

    let cel_to_kelvin = celsius_to_kelvin(temp.into());
    println!("From Celsius to Kelvin: {cel_to_kelvin}");
}

fn fahrenheit_to_celsius(temp: i16) -> i16 {
    (temp - 32) * 5 / 9
} 

fn celsius_to_fahrenheit(temp: i16) -> i16 {
    (temp * 9 / 5) + 32
}

fn celsius_to_kelvin(temp: f64) -> f64 {
    temp + 273.15
}

fn fahrenheit_to_kelvin(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0 + 273.15
}
