use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub mod player;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    Move,
    Rotate,
    Shoot,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Playing,
    GameOver,
}
