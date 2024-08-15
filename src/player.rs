use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_asset_loader::prelude::*;
use avian2d::prelude::*;

use crate::{Action, GameState};

pub fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(GameState::Loading)
            .load_collection::<PlayerAssets>(),
    );
    app.add_systems(OnEnter(GameState::Playing), spawn_player);
    app.add_systems(FixedUpdate, move_player);
    app.add_systems(Update, player_input);
}

#[derive(AssetCollection, Resource)]
struct PlayerAssets {
    #[asset(key = "image.player_sprite")]
    sprite: Handle<Image>
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Player;

fn spawn_player(mut commands: Commands, assets: Res<PlayerAssets>) {
    let input_map = InputMap::default()
        .with_axis(Action::Move, KeyboardVirtualAxis::WS)
        .with_axis(Action::Move, KeyboardVirtualAxis::VERTICAL_ARROW_KEYS)
        .with_axis(Action::Rotate, KeyboardVirtualAxis::AD)
        .with_axis(Action::Rotate, KeyboardVirtualAxis::HORIZONTAL_ARROW_KEYS)
        .with(Action::Shoot,KeyCode::Space);

    commands.spawn((
        Name::new("Player"),
        StateScoped(GameState::Playing),
        Player,
        RigidBody::Kinematic,
        Collider::circle(1.),
        SpriteBundle {
            texture: assets.sprite.clone(),
            ..default()
        },
        InputManagerBundle::with_map(input_map),
    ));
}

fn move_player() {}

fn player_input() {}


