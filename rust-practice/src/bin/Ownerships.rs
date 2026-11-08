#[derive(Clone)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let user1 = User {
        name: String::from("Alice"),
        age: 25,
    };

    let user2 = &user1;

    println!("User 1: {}", user1.name);
    println!("User 2: {}", user2.name);
}
