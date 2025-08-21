use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut x = 5; //health
    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..=100); // random number gen
    println!("I will generate a random number between (0-100) "); // info for the user

    loop{ // loop for the try

        println!("Please input your gues:");
        println!("you have {} life",x);

        let mut guess = String::new(); // store the guess value as a string

        io::stdin() // take a input
            .read_line(&mut guess)
            .expect("Faild to read line");

        let guess: u32 = match guess.trim().parse(){  // convert string to u32 value 
            Ok(num) => num,
            Err(_) => continue,
        };
        x-=1;
        

        println!("you guessed: {}",guess);
    
        match guess.cmp(&secret_number){
            Ordering::Less=> println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win");
                println!("The random number was = {}",secret_number);
                break;
            }
        }
        if x==0 {
            println!("You lose !");
            println!("the random number was = {}",secret_number);
            break;
        }
    }
}
