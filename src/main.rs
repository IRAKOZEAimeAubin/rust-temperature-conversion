use std::io;

fn main() {
    println!("HeatExchange");
    println!("Welcome to your temperature unit converter.");

    loop {
        println!("Menu:");
        println!("1. Celsius to Kelvin");
        println!("2. Kelvin to Celsius");
        println!("3. Celsius to Fahrenheit");
        println!("4. Fahrenheit to Celsius");
        println!("5. Fahrenheit to Kelvin");
        println!("6. Kelvin to Fahrenheit");
        println!("Please choose an option: ");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Enter the temperature you wish to convert: ");

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 1 {
            print_result(celsius_kelvin_conversion(choice, temperature));
            break;
        } else if choice == 2 {
            print_result(celsius_kelvin_conversion(choice, temperature));
            break;
        } else if choice == 3 {
            print_result(celsius_fahrenheit_conversion(choice, temperature));
            break;
        } else if choice == 4 {
            print_result(celsius_fahrenheit_conversion(choice, temperature));
            break;
        } else if choice == 5 {
            print_result(kelvin_fahrenheit_conversion(choice, temperature));
            break;
        } else if choice == 6 {
            print_result(kelvin_fahrenheit_conversion(choice, temperature));
            break;
        } else {
            println!("Please select an option from the provided menu!");
            break;
        }
    }
}

fn print_result(r: f32) {
    println!("Your converted temperature: {r}");
}

fn celsius_fahrenheit_conversion(c: u32, t: f32) -> f32 {
    let calculated_temperature: f32;
    if c == 3 {
        calculated_temperature = t * 9.0 / 5.0 + 32.0;
        calculated_temperature
    } else {
        calculated_temperature = t - 32.0;
        calculated_temperature / 1.8
    }
}

fn celsius_kelvin_conversion(c: u32, t: f32) -> f32 {
    let calculated_temperature: f32;
    if c == 1 {
        calculated_temperature = t + 273.15;
        calculated_temperature
    } else {
        calculated_temperature = t - 273.15;
        calculated_temperature
    }
}

fn kelvin_fahrenheit_conversion(c: u32, t: f32) -> f32 {
    let calculated_temperature: f32;
    if c == 5 {
        calculated_temperature = celsius_kelvin_conversion(1, celsius_fahrenheit_conversion(2, t));
        calculated_temperature
    } else {
        calculated_temperature = celsius_fahrenheit_conversion(3, celsius_kelvin_conversion(2, t));
        calculated_temperature
    }
}
