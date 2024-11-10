include!("user.rs");

fn main() {
    let name = String::from("Naresh Karthigeyan");
    let mut us = create_user(name);
    display_user(&mut us);
    display_user(&mut us);
}
