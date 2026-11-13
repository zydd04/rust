use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
    println!("Guess Number !");
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Enter your guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Faild to read input!");
        let guess : u32 = guess.trim().parse().expect("Please Enter number!");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You Win!"); break;}
        }
    }
}
