use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number foo");

    let secretnumber = rand::thread_rng().gen_range(1..=100);

    println!("secret number is {secretnumber}");

    let mut guess = String::new();

    
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("enter a number foo");
    println!("u guessed {guess} ai crodie ?");

    match guess.cmp(&secretnumber) {
        Ordering::Less => println!("too small gng"),
        Ordering::Greater => println!("too big foo"),
        Ordering::Equal => println!("ml crodie its {secretnumber}"),
    }

}   
