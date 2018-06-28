mod game;

fn main() {
    println!("Welcome to rustactoe!");
    let user_faction = game::get_user_faction();
    let players = game::create_players(user_faction);
    let active_player = game::pick_active_player(&players);
    let state = game::init_state(&active_player);
    println!("{} goes first", state.active_player.name);
    game::start_game(state, &players);
}
