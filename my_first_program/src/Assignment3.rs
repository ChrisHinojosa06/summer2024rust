//2. Here is to implement the function that is going to check if the guess is right
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess > secret {
        1 // guess is too high
    } else if guess < secret {
        -1 // guess is too low
    } else {
        0 // correct guess
    }
}

fn main() {
    // create a mutable variable to store a secret number
    let mut secret_number = 21; // Here is the secret number

    // going to have it go through the list of predefined guesses
    let guesses = [30, 35, 20, 21];

    // keep track of the guess count
    let mut guess_count = 0;

    // keep track if the guess is right or wrong
    let mut correct_guess = false;

    // continue the loop if the guesses are wrong and they are still within the list of numbers that I have given it
    while !correct_guess && guess_count < guesses.len() {
        // grab the current guess from the array
        let guess = guesses[guess_count];
        
        // show which one you are guessing
        println!("Guessing: {}", guess);
        
        // increase the guess count
        guess_count += 1;
        
        // call the check_guess function to check if it is the right guess
        let result = check_guess(guess, secret_number);
        //print the if the number is too high
        if result == 1 {
            println!("The number you guessed is too high");
        } else if result == -1 {
            //print if the number is too low
            println!("The number you guessed is too low!");
        } else {
            println!("You have guessed the right number");
            //set the boolean to true
            correct_guess = true; 
        }
    }
    //print a line if you are out of guesses
    if !correct_guess {
        println!("You are out of guesses");
    }
    //After the loop ends print the amount of guesses   
    println!("Number of guesses: {}", guess_count);
}
