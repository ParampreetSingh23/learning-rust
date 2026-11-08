
#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    active: bool,
}

fn main() {
    let mut users: Vec<User> = Vec::new();

    // 1. Add Alice, Bob and Charlie
    users.push(User{
     id : 1,
     name : String::from("Alice"),
     active : true,
    });
     users.push(User{
     id : 2,
     name : String::from("Bob"),
     active : true,
    });
     users.push(User{
     id : 3,
     name : String::from("Charlie"),
     active : false,
    });

    // 2. Print all users
    for user in &users {
     println!("{:?}" , user)
    }

    // 3. Print only active users
    for user in &users {
     if user.active {
      println!("{:?}",user)
     }
    }

    // 4. Print the total number of users
    println!("{}",users.len());
}
