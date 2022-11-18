use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    let secrect_number:u32 = rand::thread_rng().gen_range(1..101) // only 1 to 100 or put 1..=100 to inclusively reach 100
    loop {
        let mut guess = String::new();
        println!("Write your guess: ");
        io::stdin()  // returns a Result
            .read_line(&mut guess)
            .expect("No Input detected.");
        println!("Guess is {guess}");
        // println!("The secrect number was {secret_number}");
        let guess:u32 = match guess.trim().parse() {
            Ok(int) => int,
            Err(_) =>{ println!("expected a number but got something else.");
                        continue; },
        };
        match guess.cmp(&secrect_number){
            Ordering::Less => println!("Too smal"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => { println!("You won!");
                                break; },
        };
    }
}
