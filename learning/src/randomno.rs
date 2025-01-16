use rand::Rng;

pub fn generate_number(min: u32, max: u32) -> u32 {
  let mut rng = rand::thread_rng();
  rng.gen_range(min..=max)
}

pub fn play_guessing_game(secret_number: u32) {
    println!("Welcome to the Guessing Game!");
    println!("I'm thinking of a number between 1 and 1000. Can you guess it?");

    loop {
        // Prompt the user for a guess
        println!("Enter your guess (or 'quit' to exit):");

        // Read the user's guess
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("Failed to read guess");

        // Check if the user wants to quit
        if guess.trim().to_lowercase() == "quit" {
            println!("Thanks for playing!");
            break; // Exit the loop
        }

        // Convert the guess to a number
        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue; // Continue to the next iteration of the loop
            }
        };

        // Check if the guess is correct
        if guess_number == secret_number {
            println!("Congratulations! You guessed the number!");
            break; // Exit the loop
        } else if guess_number < secret_number {
            println!("Too low! Guess higher.");
        } else {
            println!("Too high! Guess lower.");
        }
    }
}