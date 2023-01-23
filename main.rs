use rand::Rng;
use std::cmp::Ordering;
use std::io;

//primary function
fn main() {
    println!("Guess the number!"); //will prompt the user for input

    let secret_number = rand::thread_rng().gen_range(1..=100); //generates a secret number

    let mut attempts = 5; // initialize attempts variable
    loop {
        println!("Please input your guess. You have {} attempts left", attempts);

        let mut guess = String::new(); //allows for multiple inputs
        
        io::stdin()
            .read_line(&mut guess) //reads user input
            .expect("Failed to read line"); //if something goes wrong this will print
        //allows the user to input string values
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}"); //shows user their guess
        
        //logic to decide if the user input was correct
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), //number is higher
            Ordering::Greater => println!("Too big!"), //number is lower
            Ordering::Equal => { //the correct number
                println!("You win!"); //informs the user of victory
                break;
            }
        }

        // Decrement attempts after each guess
        attempts -= 1;

        // check if attempts are over
        if attempts == 0 {
            println!("You lose!");
            break;
        }
    }
}
