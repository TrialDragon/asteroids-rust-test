use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub mod asteroid;
pub mod destruction;
pub mod game_over;
pub mod player;
pub mod projectile;
pub mod score;
pub mod stats;
pub mod viewport_bound;

pub const VIEWPORT_WIDTH: f32 = 1280.;
pub const VIEWPORT_HEIGHT: f32 = 720.;

pub const RIGHT_VIEWPORT_EDGE: f32 = VIEWPORT_WIDTH / 2.;
pub const LEFT_VIEWPORT_EDGE: f32 = -RIGHT_VIEWPORT_EDGE;
pub const TOP_VIEWPORT_EDGE: f32 = VIEWPORT_HEIGHT / 2.;
pub const BOTTOM_VIEWPORT_EDGE: f32 = -TOP_VIEWPORT_EDGE;

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
