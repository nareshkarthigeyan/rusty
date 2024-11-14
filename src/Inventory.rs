
struct Item {
    name: String,
    count: u32,
    total_count: u32,
    unlock_at_mine_level: u8,
    xp: f32,
}

struct Inventory {
    dirt: Item,
    rock: Item,
    coal: Item,
    granite: Item,
}

impl Item {
    fn create_item(nm: String, lvl: u8, xx: f32) -> Item {
        Item {
            name: nm,
            count: 0,
            total_count: 0,
            unlock_at_mine_level: lvl,
            xp: xx,
        }
    }

    fn increment_item_by(&mut self, i : u32){
        self.count += i;
        self.total_count += i;
    }

    fn decrement_item_by(&mut self, i : u32){
        if i <= self.count {
            self.count -= i;
        } else {
            println!("Not enough {} in the inventory!", self.name);
        }
    }
}

impl Inventory{
    fn create_inventory() -> Inventory {
        Inventory {
            dirt : Item::create_item("Dirt".to_string(), 0, 0.05),
            rock: Item::create_item("Rock".to_string(), 0, 0.11),
            coal: Item::create_item("Coal".to_string(), 0, 0.11),
            granite: Item::create_item("Granite".to_string(), 2, 0.55),
        }
    }
}