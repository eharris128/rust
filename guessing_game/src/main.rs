extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret # is: {}", secret_number);

    loop {
        println!("Please input a guess.");
        
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Something went wrong reading line.");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess was: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small.."),
            Ordering::Greater => println!("Too large.."),
            Ordering::Equal => {
                println!("Corret answer!");
                break;
            },
        }
    }
}
