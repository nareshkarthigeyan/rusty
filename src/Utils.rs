use std::io;
use std::io::*;

use rand::Rng;

fn get_user_char() -> char {
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Trim whitespace and return the first character, or a default character if input is empty
    input.trim().chars().next().unwrap_or('-')
}

fn rand_int(lower: i32, higher :i32) -> i32{
    rand::thread_rng().gen_range(lower..higher+1)
}