use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guessFunction(){
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Tell your guess");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)=> continue,
        };

        println!("You guessed this {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("you won");
                break;
            }
        }

    }
}