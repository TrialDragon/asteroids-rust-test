use avian2d::prelude::*;
use bevy::{color::palettes::css::GREEN, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_transform_interpolation::{RotationInterpolation, TranslationInterpolation};
use leafwing_input_manager::prelude::*;

use crate::{
    asteroid::Asteroid,
    destruction::Destroyed,
    projectile::SpawnProjectile,
    stats::{AngularAcceleration, Health, LinearAcceleration},
    viewport_bound::WrapMovement,
    Action, GameState,
};

pub fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(GameState::Loading).load_collection::<PlayerAssets>(),
    );
    app.add_systems(OnEnter(GameState::Playing), spawn_player);
    app.add_systems(FixedUpdate, move_player);
    app.add_systems(
        Update,
        (
            visualize_player_health,
            player_shoot,
            (player_destruction, collision_with_asteroid).chain(),
        ),
    );
}

#[derive(AssetCollection, Resource)]
struct PlayerAssets {
    #[asset(key = "image.player_sprite")]
    sprite: Handle<Image>,
    #[asset(key = "image.engine_exhaust")]
    engine_exhaust: Handle<Image>
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
        Health::new(3),
        RigidBody::Kinematic,
        Collider::triangle(
            Vec2::new(-30.0, -28.0),
            Vec2::new(30.0, -28.0),
            Vec2::new(0.0, 30.0),
        ),
        TranslationInterpolation,
        RotationInterpolation,
        WrapMovement,
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
    mut query: Query<
        (
            &mut LinearVelocity,
            &mut AngularVelocity,
            &ActionState<Action>,
            &LinearAcceleration,
            &AngularAcceleration,
            &Transform,
        ),
        With<Player>,
    >,
    time: Res<Time<Fixed>>,
) {
    const MAX_LINEAR_SPEED: f32 = 300.0;
    const MAX_ANGULAR_SPEED: f32 = 3.;

    for (
        mut linear_velocity,
        mut angular_velocity,
        action_state,
        linear_acceleration,
        angular_acceleration,
        transform,
    ) in &mut query
    {
        let rotation = action_state.clamped_value(&Action::Rotate);
        let direction = (transform.rotation * Vec3::Y).xy().normalize_or_zero();

        if action_state.pressed(&Action::Move) {
            // Accelerate linear velocity
            linear_velocity.0 = linear_velocity.0.move_towards(
                direction * MAX_LINEAR_SPEED,
                linear_acceleration.0 * time.delta_seconds(),
            );
        } else {
            linear_velocity.0 = linear_velocity
                .0
                .move_towards(Vec2::ZERO, linear_acceleration.0 * time.delta_seconds());
        }

        // The reason the opposite acceleration code
        // doesn't need to be in an if block
        // is because if rotation is 0 then it won't
        // accelerate, but it also won't decelerate
        // which is the purpose of this block.
        if rotation == 0.0 {
            angular_velocity.0 = angular_velocity
                .0
                .lerp(0.0, angular_acceleration.0 * time.delta_seconds());
        }

        // It appears negative rotates right and positive left
        // so this needs to be inverted to get correct rotations.
        angular_velocity.0 = angular_velocity.0.lerp(
            MAX_ANGULAR_SPEED * -rotation,
            angular_acceleration.0 * time.delta_seconds(),
        );
    }
}

fn player_shoot(
    query: Query<(&ActionState<Action>, &Transform), With<Player>>,
    mut commands: Commands,
) {
    for (action_state, transform) in &query {
        if action_state.just_pressed(&Action::Shoot) {
            const PROJECTILE_SPAWN_OFFSET: f32 = 30.;

            let direction = (transform.rotation * Vec3::Y).normalize_or_zero();
            let offset = (direction.xy() * PROJECTILE_SPAWN_OFFSET).extend(0.0);

            commands.trigger(SpawnProjectile::new(
                transform.translation + offset,
                transform.rotation,
            ))
        }
    }
}

fn player_destruction(
    mut destroyed_event_reader: EventReader<Destroyed>,
    query: Query<(), With<Player>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for Destroyed(entity) in destroyed_event_reader.read() {
        if query.contains(*entity) {
            commands.entity(*entity).despawn_recursive();
            next_state.set(GameState::GameOver);
        }
    }
}

fn collision_with_asteroid(
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut destroyed_event_writer: EventWriter<Destroyed>,
    mut player_query: Query<&mut Health, With<Player>>,
    asteroid_query: Query<(), With<Asteroid>>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        let mut logic = |first_entity: &Entity, second_entity: &Entity| {
            if player_query.contains(*first_entity) && asteroid_query.contains(*second_entity) {
                let mut health = player_query.get_mut(*first_entity).unwrap();
                health.sub(1);
                if health.current() == 0 {
                    destroyed_event_writer.send(Destroyed(*first_entity));
                }
            }
        };

        logic(entity1, entity2);
        logic(entity2, entity1);
    }
}

/// Use gizmos to render a rectangle that
/// represent the player's health.
///
/// It takes the current health (0–3; inclusive)
/// and renders a rectangle with it's base height
/// multiplied by the health so that its length
/// is relative to the health and represents it.
///
/// This should probably not use gizmos for production
/// code, but this is the simplest way to get a health
/// widget rendering for the MVP.
///
/// TODO: look into switching to a better
/// health widget rendering method?
fn visualize_player_health(
    mut gizmos: Gizmos,
    player_query: Query<(&Health, &Transform), With<Player>>,
) {
    const POSITION_OFFSET: f32 = 80.;
    const GAP_OFFSET: f32 = 4.;
    const HEALTH_SEGMENT_HEIGHT: f32 = 24.;
    const HEALTH_SEGMENT_WIDTH: f32 = 16.;

    for (health, transform) in &player_query {
        for n in 0..health.current() {
            let gizmo_position = Vec2::new(
                transform.translation.x - POSITION_OFFSET,
                transform.translation.y + (HEALTH_SEGMENT_HEIGHT + GAP_OFFSET) * n as f32,
            );

            // TODO: Make the health rotate with player?
            gizmos.rect_2d(
                gizmo_position,
                0.,
                Vec2::new(HEALTH_SEGMENT_WIDTH, HEALTH_SEGMENT_HEIGHT),
                GREEN,
            );
        }
    }
}
