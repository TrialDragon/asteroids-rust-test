use avian2d::prelude::*;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_asset_loader::prelude::*;
use bevy_transform_interpolation::*;
use leafwing_input_manager::prelude::*;

use game_library::{
    asteroid, destruction, health_pickup, player, projectile, states::{self, GameState}, stats, viewport_bound, Action, VIEWPORT_HEIGHT, VIEWPORT_WIDTH
};
use sickle_ui::SickleUiPlugin;


fn main() {
    let window_configuration = WindowPlugin {
        primary_window: Some(Window {
            title: "Asteroids Clone".to_string(),
            ..default()
        }),
        ..default()
    };

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(window_configuration));
    app.add_plugins(states::plugin);
    app.add_plugins(PhysicsPlugins::default());
    app.add_plugins(TransformInterpolationPlugin::default());
    app.add_plugins(InputManagerPlugin::<Action>::default());
    app.add_plugins(SickleUiPlugin);
    app.add_loading_state(
        LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::Title)
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("game.assets.ron"),
    );
    app.add_plugins(asteroid::plugin);
    app.add_plugins(destruction::plugin);
    app.add_plugins(health_pickup::plugin);
    app.add_plugins(player::plugin);
    app.add_plugins(projectile::plugin);
    app.add_plugins(stats::plugin);
    app.add_plugins(viewport_bound::plugin);
    app.add_systems(Startup, setup_camera);
    app.insert_resource(ClearColor(Color::srgb(0., 0., 0.)));

    // Development plugins, systems, et cetera.
    #[cfg(feature = "dev")]
    {
        app.add_plugins(PhysicsDebugPlugin::default());
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
    }

    app.run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed {
        width: VIEWPORT_WIDTH,
        height: VIEWPORT_HEIGHT,
    };
    commands.spawn((Name::new("Camera"), camera_bundle, IsDefaultUiCamera));
}
