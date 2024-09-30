use std::fs::File;
use std::io::{self, Write, Read};

// Define the Car struct
#[derive(Debug)]
struct Car {
    make: String,
    model: String,
    year: u32,
}

// Function to read car details from the console
fn reading_from_console() -> Car {
    let mut make = String::new();
    let mut model = String::new();
    let mut year = String::new();

    println!("Enter the make of your car:");
    io::stdin().read_line(&mut make).unwrap();

    println!("Enter the model of your car:");
    io::stdin().read_line(&mut model).unwrap();

    println!("Enter the year of your car:");
    io::stdin().read_line(&mut year).unwrap();

    // Trim and parse year input
    let year: u32 = year.trim().parse().unwrap();

    // Return a Car instance
    Car {
        make: make.trim().to_string(),
        model: model.trim().to_string(),
        year,
    }
}

// Function to write car details to a file
fn write_to_file(car: &Car) {
    let mut file = File::create("user_info.txt").unwrap();
    writeln!(file, "Car Information:").unwrap();
    writeln!(file, "Make: {}", car.make).unwrap();
    writeln!(file, "Model: {}", car.model).unwrap();
    writeln!(file, "Year: {}", car.year).unwrap();
}

// Function to read the content of the file and print it
fn read_from_file() {
    let mut file = File::open("user_info.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("\nThe content of the file is:\n{}", contents);
}

fn main() {
    // Read car data from the console
    let car = reading_from_console();

    // Write the car data to the file
    write_to_file(&car);

    // Read the content from the file and print it
    read_from_file();
}
