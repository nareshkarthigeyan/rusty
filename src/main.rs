include!("user.rs");
include!("Mine.rs");
include!("Player.rs");
include!("Utils.rs");


fn menu(player: &mut Player){
    print!("m) Mine\n>> ");
    std::io::stdout().flush().unwrap();
    let choice = get_user_char();
    match choice {
        'm' => {
            let mine : Mine = Mine::create_mine();
            mine.enter(player);
        },
        _ => {
            println!("Invalid Choice! Please try again!");
        }
    }
    
}
fn main() {
    let name = String::from("Naresh Karthigeyan");
    let mut player : Player = Player::create_player(name);

    println!("Entering menu:");
    loop {
        menu(&mut player);
    }    
}
