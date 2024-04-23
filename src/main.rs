use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let separator = "------ ..... ------";
    let fg_red = "\x1b[31m";
    let fg_green = "\x1b[92m";
    let reset = "\x1b[0m";

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!!!");
    println!("{separator}");

    loop {
        println!("Please, input your guess...");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!!!");

        let guess: u32 = guess.trim().parse().expect("Please type a number!...");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}{guess} too small!{}", fg_red, reset),
            Ordering::Greater => println!("{}{guess} too big!{}", fg_red, reset),
            Ordering::Equal => {
                println!("{}You win! :){}", fg_green, reset);
                break;
            }
        }

        println!("{separator}");
    }
}
