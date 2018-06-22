extern crate rand;

use rand::prelude::*;
use std::io;

struct Player {
    name: String,
    faction: String,
}

struct GameState<'a> {
    board: [&'a str; 9],
    active_player: &'a Player,
}

fn main() {
    println!("Welcome to rustactoe!");
    let user_faction = get_user_faction();
    let players = create_players(user_faction);
    let state = GameState {
        board: [" "; 9],
        active_player: pick_active_player(&players),
    };
    println!("{} goes first", state.active_player.name);
    start_game(state, &players);
}

fn get_user_faction() -> String {
    let faction = prompt("Choose your faction:").to_uppercase();
    if faction != "X" && faction != "O" {
        return get_user_faction();
    }
    println!("You've chosen {}.", faction);
    faction
}

fn prompt(message: &str) -> String {
    let mut buffer = String::new();
    println!("{}", message);
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim_right().to_string()
}

fn create_players(user_faction: String) -> [Player; 2] {
    let computer_faction = match user_faction.as_str() {
        "X" => "O".to_string(),
        "O" => "X".to_string(),
        _ => String::new()
    };
    let user = Player { name: "user".to_string(), faction: user_faction };
    let computer = Player { name: "computer".to_string(), faction: computer_faction };
    [user, computer]
}

fn pick_active_player(players: &[Player]) -> &Player {
    &players[random::<f64>().round() as usize]
}

fn start_game<'a>(mut state: GameState<'a>, players: &'a [Player]) {
    loop {
        match state.active_player.name.as_str() {
            "user" => {
                draw_board(state.board);
                let cell = get_valid_cell(&state);
                state.board[cell] = &state.active_player.faction;
                check_game_state(&state);
                state.active_player = &players[1];
            },
            "computer" => {
                println!("Computer taking turn...");
                let cell = determine_best_cell(&state);
                state.board[cell] = &state.active_player.faction;
                check_game_state(&state);
                state.active_player = &players[0];
            },
            _ => ()
        }
    }
}

fn get_valid_cell(state: &GameState) -> usize {
    let mut cell: usize = match prompt("Choose your cell (0-8):").parse() {
        Ok(cell) => cell,
        Err(_) => 9
    };

    if cell > 8 {
        println!("That's not a valid board position!");
        cell = get_valid_cell(state);
    }

    if state.board[cell] != " " {
        println!("{} has already been claimed!", cell);
        cell = get_valid_cell(state);
    }
    cell
}

fn draw_board(board: [&str; 9]) {
    println!("\n{}|{}|{}", board[0], board[1], board[2]);
    println!("-+-+-");
    println!("{}|{}|{}", board[3], board[4], board[5]);
    println!("-+-+-");
    println!("{}|{}|{}", board[6], board[7], board[8]);
}

fn determine_best_cell(state: &GameState) -> usize {
    0
}

fn check_game_state(state: &GameState) {
    let faction = &state.active_player.faction;
    println!("Checking state for {}", faction);
}
