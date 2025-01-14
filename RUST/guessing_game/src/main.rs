use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100);
    //println!("The secret number is {}", secret_number);
    println!("please input your guess");
    loop{
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess"); //when will stdin fail. 
        let guess: u32 = guess.trim().parse().expect("Please enter a number");
        println!("You guess {}",guess);
        let x = 5;
        let y = 4; 

        println!("x = {} and y = {}",x,y+2);

        match guess.cmp(& secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win!");
                break;x
            },
        }
    }
}
