include!("Inventory.rs");
struct Player {
    name : String,
    wallet : u64,
    bank_balance : u64,
    xp : u32,
    luck : u8,
    inventory : Inventory,
}

impl Player {
    fn create_player(n : String) -> Self {
        Self {
            name: n,
            wallet: 0,
            bank_balance: 0,
            xp: 0,
            luck: 1,
            inventory : Inventory::create_inventory(),
        }
    }

    fn display(&mut self){
        print!("Name: {}\nWallet: ${}\nBank Balance: ${}\nXP: {}\nLuck: {}\n", self.name, self.wallet, self.bank_balance, self.xp, self.luck);
        self.xp += 1;
    }
}