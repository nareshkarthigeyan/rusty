struct User {
    active: bool,
    name: String,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn create_user(nam: String) -> User {
        User {
            active: true,
            name: nam,
            username: String::from("nareshkarthigeyan"),
            email: String::from("nareshkarthigeyan.2005@gmail.com"),
            sign_in_count: 1,
        }
    }
    
    fn display_user(&mut self) {
        println!("Name: {}\nUsername: {}\nE-mail: {}\nSign In Count: {}\nUser Active?: {}", self.name, self.username, self.email, self.sign_in_count, self.active);
        self.sign_in_count += 1;
    }

    fn change_username(&mut self, new_username: String){
        self.username = new_username;
    }
}
