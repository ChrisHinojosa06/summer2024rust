fn sum(total: &mut i32, low: i32, high: i32) {
    // Initialize total to 0
    *total = 0;
    // Iterate from low to high (inclusive)
    for num in low..=high {
        *total += num;
    }
}

fn main() {
    // Create a variable to hold the total sum
    let mut total = 0;
    // Test the sum function for low 0 and high 100
    sum(&mut total, 0, 100);
    // Print the result, should be 5050
    println!("Total: {}", total); // Should print: "Total: 5050"
}