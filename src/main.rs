use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use game_library::player;
use game_library::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::Playing)
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                "game.assets.ron",
            ),
        )
        .run();
}
