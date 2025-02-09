
#[derive(Debug)] // you need to add this to pring out the struct directly using ":? or :#? "
struct User {
    active: bool,
    username: String,
    email:String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 131,
    };
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{user2:#?}")
}
