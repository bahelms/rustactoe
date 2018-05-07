extern crate rand;

use std::io;
use std::collections::HashMap;

// const END_GAME: u16 = 511;
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
    let mut board = HashMap::with_capacity(2);
    board.insert("x", 0b0);
    board.insert("o", 0b0);

    loop {
        let _faction = choose_faction();
        decide_first_player();
        display_board(board);
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

fn display_board(board: HashMap<&str, u16>) {
    let x_bits = format!("{:09b}", board.get("x").unwrap());
    let o_bits = format!("{:09b}", board.get("o").unwrap());
    let mut rows = [
        Vec::with_capacity(3),
        Vec::with_capacity(3),
        Vec::with_capacity(3),
    ];

    for (n, (xbit, obit)) in x_bits.chars().zip(o_bits.chars()).enumerate() {
        if xbit == '1' {
            rows[n % 3].push("X");
        } else if obit == '1' {
            rows[n % 3].push("O");
        } else {
            rows[n % 3].push(" ");
        }
    }

    let board = rows.iter()
        .map(|row| row.join("|"))
        .collect::<Vec<String>>()
        .join("\n-----\n");
    println!("\n{}", board);
}
