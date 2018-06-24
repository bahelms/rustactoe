extern crate rand;

mod ai;

use game::rand::prelude::*;
use std::collections::HashMap;
use std::io;

type Board<'a> = [&'a str; 9];

pub struct Player {
    pub name: String,
    pub faction: String,
}

#[derive(Clone)]
pub struct GameState<'a> {
    pub board: Board<'a>,
    pub active_player: &'a Player,
    pub win_states: HashMap<u8, Vec<[usize; 2]>>,
}

pub fn init_state<'a>(players: &'a [Player; 2]) -> GameState<'a> {
    GameState {
        board: [" "; 9],
        active_player: pick_active_player(players),
        win_states: init_win_states(),
    }
}

pub fn get_user_faction() -> String {
    let faction = prompt("Choose your faction:").to_uppercase();
    if faction != "X" && faction != "O" {
        return get_user_faction();
    }
    println!("You've chosen {}.", faction);
    faction
}

pub fn create_players(user_faction: String) -> [Player; 2] {
    let computer_faction = match user_faction.as_str() {
        "X" => "O".to_string(),
        "O" => "X".to_string(),
        _ => String::new()
    };
    let user = Player { name: "user".to_string(), faction: user_faction };
    let computer = Player { name: "computer".to_string(), faction: computer_faction };
    [user, computer]
}

pub fn start_game<'a>(mut state: GameState<'a>, players: &'a [Player]) {
    loop {
        match state.active_player.name.as_str() {
            "user" => {
                draw_board(&state.board);
                let cell = get_valid_cell(&state);
                state.board[cell] = &state.active_player.faction;
                match check_game_state(&state) {
                    Some(name) => {
                        draw_board(&state.board);
                        println!("{} has won the game!", name);
                        break;
                    },
                    _ => ()
                }
                state.active_player = &players[1];
            },
            "computer" => {
                println!("Computer taking turn...");
                let cell = ai::determine_best_cell(&state);
                state.board[cell] = &state.active_player.faction;
                match check_game_state(&state) {
                    Some(name) => {
                        draw_board(&state.board);
                        println!("{} has won the game!", name);
                        break;
                    },
                    _ => ()
                }
                state.active_player = &players[0];
            },
            _ => ()
        }
    }
}

fn pick_active_player(players: &[Player]) -> &Player {
    &players[random::<f64>().round() as usize]
}

fn init_win_states() -> HashMap<u8, Vec<[usize; 2]>> {
    let mut map = HashMap::new();
    map.insert(0, vec![[1, 2], [4, 8], [3, 6]]);
    map.insert(1, vec![[4, 7]]);
    map.insert(2, vec![[4, 6], [5, 8]]);
    map.insert(3, vec![[4, 5]]);
    map.insert(6, vec![[7, 8]]);
    map
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

fn draw_board(board: &Board) {
    println!("\n{}|{}|{}", board[0], board[1], board[2]);
    println!("-+-+-");
    println!("{}|{}|{}", board[3], board[4], board[5]);
    println!("-+-+-");
    println!("{}|{}|{}", board[6], board[7], board[8]);
}

fn check_game_state<'a>(state: &GameState<'a>) -> Option<(&'a str)> {
    if is_winner(&state) {
        Some(&state.active_player.name)
    } else if board_is_full(&state) {
        Some("CAT")
    } else {
        None
    }
}

fn is_winner(state: &GameState) -> bool {
    let position = match first_position(&state) {
        Some(position) => position,
        None => return false
    };
    match state.win_states.get(&position) {
        Some(win_states) => matches_win_state(&win_states, state),
        None => false
    }
}

fn matches_win_state(win_states: &Vec<[usize; 2]>, state: &GameState) -> bool {
    let mut match_count = 0;
    for &win_state in win_states {
        for &index in win_state.iter() {
            if state.board[index] == state.active_player.faction {
                match_count += 1;
            }
        }
    }
    match_count == 2
}

fn first_position(state: &GameState) -> Option<u8> {
    let mut first_position = None;
    for (position, &contents) in state.board.iter().enumerate() {
        if contents == state.active_player.faction {
            first_position = Some(position as u8);
            break;
        }
    }
    first_position
}

fn board_is_full(state: &GameState) -> bool {
    for cell in state.board.iter() {
        if *cell != " " {
            return false;
        }
    }
    true
}

fn prompt(message: &str) -> String {
    let mut buffer = String::new();
    println!("{}", message);
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim_right().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_winner_with_empty_board() {
        let state = GameState {
            board: ["", "", "", "", "", "", "", "", ""],
            active_player: &Player { name: "Bob".to_string(), faction: "X".to_string() },
            win_states: init_win_states(),
        };
        assert_eq!(is_winner(&state), false);
    }

    #[test]
    fn test_is_winner_048_diagonal_position() {
        let state = GameState {
            board: ["X", "", "", "", "X", "", "", "", "X"],
            active_player: &Player { name: "Bob".to_string(), faction: "X".to_string() },
            win_states: init_win_states(),
        };
        assert_eq!(is_winner(&state), true);
    }

    #[test]
    fn test_is_winner_258_diagonal_position() {
        let state = GameState {
            board: ["", "", "X", "", "", "X", "", "", "X"],
            active_player: &Player { name: "Bob".to_string(), faction: "X".to_string() },
            win_states: init_win_states(),
        };
        assert_eq!(is_winner(&state), true);
    }

    #[test]
    fn test_first_position_returns_index_of_first_instance_of_faction() {
        let state = GameState {
            board: ["X", "", "", "", "", "", "", "", ""],
            active_player: &Player { name: "Bob".to_string(), faction: "X".to_string() },
            win_states: init_win_states(),
        };
        assert_eq!(first_position(&state), Some(0));
    }
}
