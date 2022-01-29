use rand::Rng;
use std::cmp::Ordering;
use std::io; // Comparing data given

fn main() {
    println!("Please guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("[INPUT] Number: ");

        let mut guess = String::new(); // this value is allowed to be mutated, hence `mut`

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            // parse will return a Result, which is an enumeration of what can happen
            Ok(num) => num, // succesfully parsed
            Err(_) => continue, // not a number and we just ignore
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
