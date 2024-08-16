use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub mod asteroid;
pub mod destruction;
pub mod player;
pub mod projectile;
pub mod score;
pub mod stats;
pub mod viewport_bound;

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
