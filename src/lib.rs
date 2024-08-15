use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub mod player;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    Move,
    Rotate,
    Shoot,
}

impl Actionlike for Action {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Action::Move => InputControlKind::Axis,
            Action::Rotate => InputControlKind::Axis,
            Action::Shoot => InputControlKind::Button,
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
