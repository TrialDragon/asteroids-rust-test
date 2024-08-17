use bevy::prelude::*;

use crate::GameState;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::GameOver), restart_game);
}

fn restart_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}
