use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("secret number: {}", secret_number);
    println!("Guess the secret number:");
    loop{

        let mut guess = "".to_owned();
        io::stdin().read_line(&mut guess).expect("Error reading user input");
        
        let guess: i32 = match guess.trim().parse(){
            Ok(x) => x,
            Err(_) => {
                println!("No string allowed only numbers");
                continue;
            }
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}","you win".green());
                break;
            },
            Ordering::Greater => println!("{}","too large\ntry again:".red()),
            Ordering::Less => println!("{}","too small\ntry again:".red())     
        };
    }
}




