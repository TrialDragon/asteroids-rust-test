pub use bevy::prelude::*;

mod game_over;
mod playing;
mod title;

pub fn plugin(app: &mut App) {
    app.add_plugins(game_over::plugin);
    app.add_plugins(playing::plugin);
    app.add_plugins(title::plugin);
    app.init_state::<GameState>();
    app.enable_state_scoped_entities::<GameState>();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Title,
    Playing,
    GameOver,
}
