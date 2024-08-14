use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use game_library::player;
use game_library::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .run();
}
