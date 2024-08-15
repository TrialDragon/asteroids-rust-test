use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_transform_interpolation::{RotationInterpolation, TranslationInterpolation};
use leafwing_input_manager::prelude::*;

use crate::{stats::{LinearAcceleration, AngularAcceleration}, Action, GameState};

pub fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(GameState::Loading)
            .load_collection::<PlayerAssets>(),
    );
    app.add_systems(OnEnter(GameState::Playing), spawn_player);
    app.add_systems(FixedUpdate, move_player);
    // This is in Update instead of FixedUpdate
    // because transform interpolation won't allow
    // the teleporting needed in FixedUpdate, only
    // non-fixed-timestep schedules.
    app.add_systems(Update, player_movement_wrapping);
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
        .with_axis(Action::Rotate, KeyboardVirtualAxis::AD)
        .with_axis(Action::Rotate, KeyboardVirtualAxis::HORIZONTAL_ARROW_KEYS)
        .with(Action::Shoot, KeyCode::Space)
        .with(Action::Move, KeyCode::KeyW)
        .with(Action::Move, KeyCode::ArrowUp);

    commands.spawn((
        Name::new("Player"),
        StateScoped(GameState::Playing),
        Player,
        RigidBody::Kinematic,
        Collider::circle(1.),
        TranslationInterpolation,
        RotationInterpolation,
        LinearAcceleration(150.),
        AngularAcceleration(2.),
        SpriteBundle {
            texture: assets.sprite.clone(),
            ..default()
        },
        InputManagerBundle::with_map(input_map),
    ));
}

// Need to allow this, otherwise trying to
// break up the type to make it less complex
// breaks adding the system to the app.
#[allow(clippy::type_complexity)]
fn move_player(
    mut query: Query<(
        &mut LinearVelocity,
        &mut AngularVelocity,
        &ActionState<Action>,
        &LinearAcceleration,
        &AngularAcceleration,
        &Transform,
    ),
        With<Player>>,
    time: Res<Time<Fixed>>
) {
    const MAX_LINEAR_SPEED: f32 = 300.0;
    const MAX_ANGULAR_SPEED: f32 = 3.;

    for (mut linear_velocity, mut angular_velocity, action_state, linear_acceleration, angular_acceleration, transform) in &mut query {
        let rotation = action_state.clamped_value(&Action::Rotate);
        let direction = transform.rotation * Vec3::Y;

        if action_state.pressed(&Action::Move) {
            // Accelerate linear velocity 
            linear_velocity.0 = linear_velocity.0.move_towards(direction.xy() * MAX_LINEAR_SPEED, linear_acceleration.0 * time.delta_seconds());
        } else {
            linear_velocity.0 = linear_velocity.0.move_towards(Vec2::ZERO, linear_acceleration.0 * time.delta_seconds());
        }

        // The reason the opposite acceleration code
        // doesn't need to be in an if block
        // is because if rotation is 0 then it won't
        // accelerate, but it also won't decelerate
        // which is the purpose of this block.
        if rotation == 0.0 {
            angular_velocity.0 = angular_velocity.0.lerp(0.0, angular_acceleration.0 * time.delta_seconds());
        }

        // It appears negative rotates right and positive left
        // so this needs to be inverted to get correct rotations.
        angular_velocity.0 = angular_velocity.0.lerp(MAX_ANGULAR_SPEED * -rotation, angular_acceleration.0 * time.delta_seconds());
    }
}

fn player_movement_wrapping(mut query: Query<&mut Transform, With<Player>>) {
    for mut transform in &mut query {
        if transform.translation.x >= 640. {
            transform.translation.x = -639.;
        } else if transform.translation.x <= -640. {
            transform.translation.x = 639.;
        }

        if transform.translation.y >= 360. {
            transform.translation.y = -359.;
        } else if transform.translation.y <= -360. {
            transform.translation.y = 359.;
        }
    }
}
