extern crate rand;

use std::io;

// const END_GAME: u16 = 511;
// let mut x_positions: u16 = 0b0;
// let mut o_positions: u16 = 0b0;
// x_positions |= 0b000_010_000;
// o_positions |= 0b100_000_000;
// x_positions |= 0b000_000_001;

fn main() {
    print_title();
    game_loop();
}

fn print_title() {
    println!("###################################");
    println!("#### Tic-Tac-Toe Battleground! ####");
    println!("###################################\n");
}

fn game_loop() {
    loop {
        let _faction = choose_faction();
        decide_first_player();
        println!("\nDisplay board here...");
        break;
    }
}

fn prompt(message: &str) -> String {
    let mut input = String::new();
    println!("{}", message);
    io::stdin().read_line(&mut input).expect("Failed to read line!");
    input.trim().to_string()
}

fn toss_coin() -> u8 {
    let (num, _) = rand::random::<(f64, char)>();
    num.round() as u8
}

fn choose_faction() -> String {
    let mut faction = prompt("Choose your faction: X, O").to_lowercase();
    while faction != "x" && faction != "o" {
        println!("Unknown value. Try again");
        faction = choose_faction();
    }
    faction
}

fn decide_first_player() {
    let coin_side = prompt("Choose heads or tails:");
    let result = match toss_coin() {
        0 => "tails",
        1 => "heads",
        _ => ""
    };

    print!("Coin toss is {} -- ", result);
    if result == coin_side {
        println!("You go first!");
    } else {
        println!("Machine goes first!");
    }
}
