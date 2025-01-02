fn main() {
    let mut s = String::from("Hello");
    println!("{}",change(&mut s));
    // dangle();
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5a
    println!("The first word is {word}");

    s.clear(); // this empties the String, making it equal to ""

    //println!("The first word is {word}");
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn change(some_string: &mut String)->usize{
    some_string.push_str(", World");
    some_string.len()
}
// fn dangle()-> String{
//     let s = String::from("Hello");
//     s
// }