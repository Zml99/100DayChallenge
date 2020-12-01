use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number number!");

    //Generates random number from 1 to 100
    let secrect_number = rand::thread_rng().gen_range(1, 101);

    loop{

        println!("Please input a number.");
        
        //Creates empty string
        let mut guess = String::new();
        
        //Takes input from the user and store it in guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        //Convert guess into a u32 type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secrect_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
