
struct Mine {
    mine_levels_at_mine_attempts: [u32; 12],
}

impl Mine {
    fn create_mine() -> Self {
        Self {
            mine_levels_at_mine_attempts : [0, 55, 128, 377, 695, 889, 780, 1001, 2350, 3978, 5112, 7500],
        }
    }

    fn get_mine_level(&self, times_mined: u16) -> u16 {
        let mut level = 0;
        for (i, &attempts) in self.mine_levels_at_mine_attempts.iter().enumerate() {
            if times_mined >= attempts as u16 {
                level = i as u16;
            } else {
                break;
            }
        }
        level
    }

    fn mine(player: &mut Player, _level: u16){
        println!("mining.....");
        std::io::stdout().flush().unwrap();

        let i : i32 = rand_int(0, 1000);
        let count = rand_int(0, 7);
        if i % 2 == 0 {
            print!("You got {} pieces of dirt", count);
            player.inventory.dirt.increment_item_by(count as u32);
        }
        else if i % 3 == 0 {
            print!("You got {} pieces of coal", count);
            player.inventory.coal.increment_item_by(count as u32);
        }
        else {
            print!("You got {} pieces of granite", count);
            player.inventory.granite.increment_item_by(count as u32);
        }
        player.times_mined += 1;
    }

    fn enter(&self, player: &mut Player) {
        println!("Entering mine...\nClick 'm' to mine");
        std::io::stdout().flush().unwrap();
        loop {
            let times_mined = player.times_mined;
            let level = self.get_mine_level(times_mined);
            print!(">> ");
            std::io::stdout().flush().unwrap();
            let c = get_user_char();

            println!("got {} ", c);

            match c {
                'm' => {
                    println!("Minint....");
                     Mine::mine(player, level); 
                }
                _ => {break;}
            }
            //mining logic

        }
    }

}