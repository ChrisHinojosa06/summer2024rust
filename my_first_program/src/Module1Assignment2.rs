// Function to check if a number is even
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // 1. Create an array with 10 numbers
    let int_numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 2. Iterate through the array using a for loop
    for &num in int_numbers.iter() {
    // 3. Check if the number is even or odd using the is_even function
        if is_even(num) {
            println!("The number is even");
        } else {
            println!("The number is odd");
        }
    // the reason that im putting this above the other 2 is for the fact that in logic
    //i would have it check if they are equal to both if not then we can check the others sepretely
    // 3 Check if the number is divisible by both 3 and 5 and print "FizzBuzz"
    //if both of the numbers are divisible 
        if num % 3 == 0 && num % 5 == 0 {
            //print
            println!("FizzBuzz");
        }
        // 3. Check if the number is divisible by 3 and print "Fizz" 
        else if num % 3 == 0 {
            //print
            println!("Fizz");
        }
         // 3. Check if the number is divisible by 5 and print "Buzz"
         else if num % 5 == 0 
         {
            //print
            println!("Buzz");
        }
    }

    // 4. Calculate the sum of the array using a while loop
    //we are going to be declaring it with a 0 so we dont garbage being assigned to it 
    let mut sum = 0;
    //same concept here
    let mut index = 0; 
    //We are going to make a  while statement that is going to find the sum of all the numbers
    while index < int_numbers.len() {
        //add each number in the index
        sum += int_numbers[index];
        //itterate through the array
        index += 1;
    }
    println!("Sum = {}", sum);

    // 5. Find the largest number in the array using a loop
    //we are going to first start at the beginning of the array and make it our default max until we find a new one
    let mut bignum = int_numbers[0];
    //reset it to the beginning
    index = 1;
    while index < int_numbers.len() {
        //if the nunber is bigger than the old one
        if int_numbers[index] > bignum{
            //then assign it to the new one
            bignum = int_numbers[index];
        }
        //iterate through the list
        index += 1;
    }
    println!("The largest number in the array is: {}", bignum);
}