//use std::cmp::Ordering;
//use std::io;

fn main() {
    // println!("Hello, world!");
    // let mut x = 5 ;
    // println!("the value of x is {x}");
    // x = 6;
    // println!("the value of x is {x}");

    //const THREE_HOURS_SECONDS:u32 = 3*60*60; // Naming convention of const in rust is caps with _

    let x = 5;
    //shadowing is different from mut as we can technically change the type of x at the next time we assign. 
    let x = x+1;// because we are using the let key word the variable is reassigned to a different memory address?
    {
        let x = 23;
        println!("the value of x is {x}");
    }
    println!("the value of x is {x}");

    //Datatypes in RUST 
    let guess:u32 = "42".parse().expect("Not a number.");
    println!("the guess value is {guess}");

    another_function();
    five();

    let y = {
        let x = 3;
        let x= x + 7;
        x // No semicolon as it does
    };

    println!("The value of y is: {y}");

    let s1 = String::from("hello");
    //let s1 = s2; shallow copy 
    let s2 = s1.clone();

    println!("{s1} {s2}, world!");
}

fn another_function(){
    println!("Another function.");
}
fn five() -> i32 {
    5
}
