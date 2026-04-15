use std::io;

const USERS: [&str; 2] = ["guest", "root"];

fn authenticate(user: &str) {
    println!("User: {user}");
    let mut existing: bool = false;
    for u in USERS {
        if u == user {existing = true; break}
    }
    if existing {println!("Sucessfully authenticated as {user}");} else {println!("User not existent");}
}

fn main() {
    let mut user = String::new();
    println!("Say your username");
    io::stdin().read_line(&mut user).expect("Unknown error while reading input");
    let user = user.trim().to_lowercase();
    authenticate(&user);
}
