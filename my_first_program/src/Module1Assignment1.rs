// Declare a constant for the freezing point of water in Fahrenheit 32F
const FREEZING_POINT_F: f64 = 32.0;

// Fahrenheit to Celsius function
fn ftoc(temp_f: f64) -> f64 {
    (temp_f - FREEZING_POINT_F) * (5.0 / 9.0)
}

// Celsius to Fahrenheit function
fn ctof(temp_c: f64) -> f64 {
    (temp_c * (9.0 / 5.0)) + FREEZING_POINT_F
}

fn main() {
    // First, create a mutable variable for the temperature in Fahrenheit
    let mut temp_f = 48.0;

    // Convert it to Celsius using the function and print the result
    let temp_c = ftoc(temp_f);
    println!("{}°F is {:.2}°C", temp_f, temp_c);

    // Use a loop to convert and print the next 5 integer temperatures
    for _ in 0..5 {
        temp_f += 1.0;
        let temp_c = ftoc(temp_f);
        println!("{}°F is {:.2}°C", temp_f, temp_c);
    }
}