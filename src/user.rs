struct User {
    active: bool,
    name: String,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn create_user(nam: String) -> User {
    let user1 = User {
        active: true,
        name: nam,
        username: String::from("nareshkarthigeyan"),
        email: String::from("nareshkarthigeyan.2005@gmail.com"),
        sign_in_count: 1,
    };
    return user1;
}

fn display_user(u: &mut User) {
    println!("Name: {}\nUsername: {}\nE-mail: {}\nSign In Count: {}\nUser Active?: {}", u.name, u.username, u.email, u.sign_in_count, u.active);
    u.sign_in_count += 1;
}