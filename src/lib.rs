use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub mod player;
pub mod stats;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Move,
    Rotate,
    Shoot,
}

impl Actionlike for Action {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Action::Rotate => InputControlKind::Axis,
            Action::Move | Action::Shoot => InputControlKind::Button,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Playing,
    GameOver,
}
