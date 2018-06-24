use game::{Board, GameState, check_game_state};

pub fn determine_best_cell(state: &GameState) -> usize {
    let open_cells = open_cells(&state.board);

    match can_win_in_next_move(&open_cells, &state.active_player.faction, &state) {
        Some(cell) => return cell,
        None => ()
    }

    let opposing_faction = opposing_faction(&state.active_player.faction);
    match can_win_in_next_move(&open_cells, opposing_faction, &state) {
        Some(cell) => return cell,
        None => ()
    }

    // attempt center
    if state.board[4] == " " {
        return 4;
    }

    // attempt corner
    let corners = [0, 2, 6, 8];
    for &corner in corners.iter() {
        if state.board[corner] == " " {
            return corner;
        }
    }

    // take first free side cell
    let sides = [1, 3, 5];
    for &side in sides.iter() {
        if state.board[side] == " " {
            return side;
        }
    }
    7
}

fn open_cells(board: &Board) -> Vec<usize> {
    let mut open_cells = Vec::with_capacity(9);
    for (index, &cell) in board.iter().enumerate() {
        if cell == " " {
            open_cells.push(index);
        }
    }
    open_cells
}

fn can_win_in_next_move(cells: &Vec<usize>, faction: &str, state: &GameState) -> Option<usize> {
    for &cell in cells {
        let mut test_state = state.clone();
        test_state.board[cell] = faction;
        match check_game_state(&test_state) {
            Some(_) => return Some(cell),
            None => ()
        };
    }
    None
}

fn opposing_faction(faction: &str) -> &str {
    if faction == "X" {
        "O"
    } else {
        "X"
    }
}
