use std::io;
use std::cmp::Ordering;
use rand::Rng;

/* Author: Astitv Shandilya [15/12/2020] */

fn main() {
    println!("Guess the Number!");

    let secret = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is {}", secret);

    loop {
        //create a mutable variable;
        println!("Enter your guess below:");
        let mut guess = String::new(); 

        //read the input in guess var;
        io::stdin()
        .read_line(&mut guess) 
        .expect("Failed to read guess!");

        //convert to input string to number; 
        let guess :u32 = match guess.trim().parse() {
            //check and continue for wrong input;
            Ok(num)=> num,
            Err(_) => continue,
        };

        //println!("The guess is {}", guess);

        match guess.cmp(&secret){
            Ordering::Less => println!("Undershoot!"),
            Ordering::Greater => println!("Overshoot!"),
            Ordering::Equal => {
                //exit the game on right answer;
                println!("Congrats! Right Answer!");
                break;
            },
        }
    }
}
