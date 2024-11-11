struct Mine {
    mine_levels_at_mine_attempts: [u32; 12],
}

impl Mine {
    fn enter() -> Self {
        Self {
            mine_levels_at_mine_attempts : [0, 55, 128, 377, 695, 889, 780, 1001, 2350, 3978, 5112, 7500],
        }
    }
}