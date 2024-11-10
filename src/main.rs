include!("user.rs");

fn main() {
    let name = String::from("Naresh Karthigeyan");
    let mut us = User::create_user(name);
    println!("");
    us.display_user();  
    us.change_username(String::from("nareshkarthigeyan_2"));
    println!("\nAfter username change\n");
    us.display_user();
    println!();
}
