include!("user.rs");
include!("Mine.rs");
include!("Player.rs");

fn main() {
    let name = String::from("Naresh Karthigeyan");
    let mut player : Player = Player::create_player(name);
    for i in 1..=15000{
        player.inventory.coal.increment_item_by(5);
        println!("{} : {}", player.inventory.coal.name, player.inventory.coal.count);
    }
}
