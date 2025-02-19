use asteroid_spawner::SpawnAsteroids;
use avian2d::{math::PI, prelude::*};
use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;
use bevy_transform_interpolation::*;
use rand::{thread_rng, Rng};

use crate::{
    destruction::Destroyed, health_pickup::SpawnHealthPickup, projectile::{Shootable, Shot}, states::GameState, stats::{AngularAcceleration, Health, LinearAcceleration, Points, Score}, viewport_bound::DestroyOutOfBounds
};

pub mod asteroid_spawner;

pub fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(GameState::Loading).load_collection::<AsteroidAssets>(),
    );
    app.insert_resource(AsteroidID(0));
    app.init_resource::<SmallAsteroidMap>();
    app.register_type::<SmallAsteroidMap>();
    app.register_type::<AsteroidID>();
    app.observe(spawn_asteroid);
    app.add_systems(FixedUpdate, move_asteroids);
    app.add_systems(Update, (shot_asteroids, destroyed_asteroids).chain());
    app.add_plugins(asteroid_spawner::plugin);
}

#[derive(AssetCollection, Resource)]
struct AsteroidAssets {
    #[asset(key = "image.basic_asteroid")]
    basic_asteroid: Handle<Image>,
    #[asset(key = "image.small_basic_asteroid")]
    small_basic_asteroid: Handle<Image>,
    #[asset(key = "image.advanced_asteroid")]
    advanced_asteroid: Handle<Image>,
    #[asset(key = "image.small_advanced_asteroid")]
    small_advanced_asteroid: Handle<Image>,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct AsteroidID(pub usize);

impl AsteroidID {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&mut self) -> usize {
        let value = self.0;
        // Increment the ID so that the next spawned
        // asteroid has its own unique ID.
        //
        // We use a wrapping (overflowing) add
        // because on the unlikely off chance that
        // the ID's get that far we can presume it's
        // safe to go back to the start.
        // If an asteroid of ID 0, or some other low number,
        // still exists at that point, then something
        // has gone terribly wrong with de-spawning
        // or the user's PC is on fire from simply
        // spawning too many entities at once.
        self.0 = self.0.wrapping_add(1);
        value
    }
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
struct SmallAsteroidMap(HashMap<usize, u16>);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Asteroid {
    id: usize,
    direction: Vec3,
}

#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
enum AsteroidKind {
    Basic,
    SmallBasic,
    Advanced,
    SmallAdvanced,
}

impl AsteroidKind {
    pub fn get_name(&self) -> String {
        String::from(match self {
            AsteroidKind::Basic => "BasicAsteroid",
            AsteroidKind::SmallBasic => "SmallBasicAsteroid",
            AsteroidKind::Advanced => "AdvancedAsteroid",
            AsteroidKind::SmallAdvanced => "SmallAdvancedAsteroid",
        })
    }
    pub fn get_health(&self) -> u16 {
        match self {
            AsteroidKind::Basic | AsteroidKind::SmallBasic => 1,
            AsteroidKind::Advanced | AsteroidKind::SmallAdvanced => 3,
        }
    }

    fn get_texture(&self, assets: Res<AsteroidAssets>) -> Handle<Image> {
        match self {
            AsteroidKind::Basic => assets.basic_asteroid.clone(),
            AsteroidKind::Advanced => assets.advanced_asteroid.clone(),
            AsteroidKind::SmallBasic => assets.small_basic_asteroid.clone(),
            AsteroidKind::SmallAdvanced => assets.small_advanced_asteroid.clone(),
        }
    }

    fn get_collider_radius(&self) -> f32 {
        match self {
            AsteroidKind::Basic | AsteroidKind::Advanced => 28.,
            AsteroidKind::SmallBasic | AsteroidKind::SmallAdvanced => 14.,
        }
    }

    fn is_smaller(&self) -> bool {
        match self {
            AsteroidKind::Basic | AsteroidKind::Advanced => false,
            AsteroidKind::SmallBasic | AsteroidKind::SmallAdvanced => true,
        }
    }

    fn get_smaller(&self) -> Self {
        match self {
            AsteroidKind::Basic | AsteroidKind::SmallBasic => AsteroidKind::SmallBasic,
            AsteroidKind::Advanced | AsteroidKind::SmallAdvanced => AsteroidKind::SmallAdvanced,
        }
    }
}

#[derive(Event, Debug)]
pub struct SpawnAsteroid {
    kind: AsteroidKind,
    transform: Transform,
    direction: Vec3,
    id: usize,
}

impl SpawnAsteroid {
    fn new(kind: AsteroidKind, transform: Transform, direction: Vec3, id: usize) -> Self {
        Self {
            kind,
            transform,
            direction,
            id,
        }
    }
}

fn spawn_asteroid(
    trigger: Trigger<SpawnAsteroid>,
    mut commands: Commands,
    assets: Res<AsteroidAssets>,
) {
    let event = trigger.event();

    commands.spawn((
        Name::new(event.kind.get_name()),
        event.kind,
        StateScoped(GameState::Playing),
        Asteroid {
            id: event.id,
            direction: event.direction,
        },
        DestroyOutOfBounds,
        Shootable,
        Health::new(event.kind.get_health()),
        Points(1),
        SpriteBundle {
            transform: event.transform,
            texture: event.kind.get_texture(assets),
            ..default()
        },
        RigidBody::Kinematic,
        // TODO: Add a `#get_collider_radius()` method
        // to AsteroidKind.
        Collider::circle(event.kind.get_collider_radius()),
        TranslationInterpolation,
        RotationInterpolation,
        LinearAcceleration(110.),
        AngularAcceleration(1.0),
    ));
}

fn move_asteroids(
    mut query: Query<(
        &Asteroid,
        &LinearAcceleration,
        &AngularAcceleration,
        &mut LinearVelocity,
        &mut AngularVelocity,
    )>,
    time: Res<Time<Fixed>>,
) {
    const LINEAR_MAX_SPEED: f32 = 200.;
    const ANGULAR_MAX_SPEED: f32 = PI;

    for (
        asteroid,
        linear_acceleration,
        angular_acceleration,
        mut linear_velocity,
        mut angular_velocity,
    ) in &mut query
    {
        let target_velocity = (asteroid.direction * LINEAR_MAX_SPEED).xy();

        linear_velocity.0 = linear_velocity.0.move_towards(
            target_velocity,
            linear_acceleration.0 * time.delta_seconds(),
        );

        angular_velocity.0 = angular_velocity.0.lerp(
            ANGULAR_MAX_SPEED,
            angular_acceleration.0 * time.delta_seconds(),
        );
    }
}

fn destroyed_asteroids(
    mut event_reader: EventReader<Destroyed>,
    asteroid_query: Query<(&Health, &Points, &AsteroidKind, &Transform, &Asteroid)>,
    mut score: ResMut<Score>,
    mut small_asteroid_map: ResMut<SmallAsteroidMap>,
    mut commands: Commands,
) {
    let mut rng = thread_rng();
    for Destroyed(entity) in event_reader.read() {
        if asteroid_query.contains(*entity) {
            let (health, points, kind, transform, asteroid) = asteroid_query.get(*entity).unwrap();
            if health.current() == 0 {
                score.0 += points.0;
                let spawn_health: bool = rng.gen();

                if kind.is_smaller() && spawn_health {
                    commands.trigger(SpawnHealthPickup::new(transform.translation));
                } else if !kind.is_smaller() {
                    for n in -1..=1 {
                        const SMALL_ASTEROID_OFFSET: f32 = 90.;

                        let mut new_direction = Transform::from_translation(asteroid.direction);
                        new_direction.rotate_z(n as f32 * (PI / 8.));
                        let new_direction = new_direction
                            .rotation
                            .mul_vec3(new_direction.translation)
                            .normalize_or_zero();

                        let mut new_transform = *transform;
                        new_transform.translation += new_direction * SMALL_ASTEROID_OFFSET;

                        commands.trigger(SpawnAsteroid::new(
                            kind.get_smaller(),
                            new_transform,
                            new_direction,
                            asteroid.id,
                        ));
                    }
                }
            } else if !kind.is_smaller() {
                commands.trigger(SpawnAsteroids::new(1));
            }

            if kind.is_smaller() {
                let value = small_asteroid_map.0.entry(asteroid.id).or_insert(0);
                *value += 1;
                if *value >= 3 {
                    commands.trigger(SpawnAsteroids::new(1));
                    small_asteroid_map.0.remove(&asteroid.id);
                }
            }

            commands.entity(*entity).despawn_recursive();
        }
    }
}

fn shot_asteroids(
    mut shot_event_reader: EventReader<Shot>,
    mut destroyed_event_writer: EventWriter<Destroyed>,
    mut asteroid_query: Query<&mut Health, With<Asteroid>>,
) {
    for Shot(entity) in shot_event_reader.read() {
        if asteroid_query.contains(*entity) {
            let mut health = asteroid_query.get_mut(*entity).unwrap();
            health.sub(1);
            if health.current() == 0 {
                destroyed_event_writer.send(Destroyed(*entity));
            }
        }
    }
}
