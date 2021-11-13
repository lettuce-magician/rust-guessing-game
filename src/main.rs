mod game;
use game::Game;
use rand::Rng;

fn main() {
    let mut new_game = Game {
        secret_number: rand::thread_rng().gen_range(1..=100),
    };
    new_game.start();
}