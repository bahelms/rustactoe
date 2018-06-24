mod game;

fn main() {
    println!("Welcome to rustactoe!");
    let user_faction = game::get_user_faction();
    let players = game::create_players(user_faction);
    let state = game::init_state(&players);
    println!("{} goes first", state.active_player.name);
    game::start_game(state, &players);
}
