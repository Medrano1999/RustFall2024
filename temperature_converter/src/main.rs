fn fahrenheit_to_celsius(f: f64) -> f64{
    (f-32.0)*(5.0/9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    (c*(9.0/5.0))+32.0
}

fn main() {
    let mut fahrenheit:f64 = 100.0;
    let mut counter = 0;
    loop{
        println!("{} degrees fahrenheit is equal to {} degrees celcius", fahrenheit, fahrenheit_to_celsius(fahrenheit));
        fahrenheit += 1.0;
        counter += 1;
        if counter == 6{
            break;
        }
    }
}
