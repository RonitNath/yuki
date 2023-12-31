pub mod setup;
pub mod prelude;
pub mod controls;
pub mod config;
pub mod logic;
pub mod game;

use rand::Rng;

use crate::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            game::GamePlugin,
            logic::LogicPlugin,
            setup::SetupPlugin,
            controls::ControlsPlugin,
        ))
        .run();
}

#[test]
pub fn dice() {
    // roll a 20-sided dice

    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(1..=20);

    println!("You rolled a {}", roll);
}
