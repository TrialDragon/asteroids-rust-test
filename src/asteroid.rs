use avian2d::{math::PI, prelude::*};
use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;
use bevy_transform_interpolation::*;
use rand::{seq::IteratorRandom, thread_rng, Rng};

use crate::{
    destruction::Destroyed, projectile::{Shootable, Shot}, score::Score, stats::{AngularAcceleration, Health, LinearAcceleration, Points}, viewport_bound::DestroyOutOfBounds, GameState
};

pub fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(GameState::Loading).load_collection::<AsteroidAssets>(),
    );
    app.insert_resource(AsteroidID(0));
    app.init_resource::<SmallAsteroidMap>();
    app.register_type::<SmallAsteroidMap>();
    app.register_type::<AsteroidID>();
    app.observe(spawn_asteroid);
    app.observe(spawn_asteroids);
    app.add_systems(
        OnEnter(GameState::Playing),
        (setup_asteroid_spawners, initial_spawn_asteroids).chain(),
    );
    app.add_systems(FixedUpdate, move_asteroids);
    app.add_systems(Update, (shot_asteroids, destroyed_asteroids).chain());
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
struct Asteroid {
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
        String::from(
            match self {
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

    fn get_points(&self) -> u32 {
        match self {
            AsteroidKind::Basic | AsteroidKind::SmallBasic => 1,
            AsteroidKind::Advanced | AsteroidKind::SmallAdvanced => 5,
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
        Points(event.kind.get_points()),
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
        LinearAcceleration(50.),
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
    const LINEAR_MAX_SPEED: f32 = 100.;
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

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
struct AsteroidSpawner {
    normal_direction: Vec3,
}

impl AsteroidSpawner {
    pub fn new(direction: Vec3) -> Self {
        Self {
            normal_direction: direction,
        }
    }
}

#[derive(Bundle)]
struct AsteroidSpawnerBundle {
    name: Name,
    state_scoped: StateScoped<GameState>,
    spawner: AsteroidSpawner,
    transform_bundle: TransformBundle,
}

impl Default for AsteroidSpawnerBundle {
    fn default() -> Self {
        Self {
            name: Name::new("AsteroidSpawner"),
            state_scoped: StateScoped(GameState::Playing),
            spawner: AsteroidSpawner::new(Vec3::ZERO),
            transform_bundle: TransformBundle::default(),
        }
    }
}

impl AsteroidSpawnerBundle {
    pub fn new(position: Vec3, direction: Vec3) -> Self {
        Self {
            spawner: AsteroidSpawner::new(direction),
            transform_bundle: TransformBundle::from_transform(Transform::from_translation(
                position,
            )),
            ..default()
        }
    }
}

fn setup_asteroid_spawners(mut commands: Commands) {
    const OFFSET: f32 = 40.;

    const RIGHT_VIEWPORT_EDGE: f32 = 640. + OFFSET;
    const LEFT_VIEWPORT_EDGE: f32 = -640. - OFFSET;
    const TOP_VIEWPORT_EDGE: f32 = 360. + OFFSET;
    const BOTTOM_VIEWPORT_EDGE: f32 = -360. - OFFSET;

    let spawner_points: Vec<Vec3> = vec![
        // Corners.
        Vec3::new(RIGHT_VIEWPORT_EDGE, TOP_VIEWPORT_EDGE, 0.0),
        Vec3::new(RIGHT_VIEWPORT_EDGE, BOTTOM_VIEWPORT_EDGE, 0.0),
        Vec3::new(LEFT_VIEWPORT_EDGE, TOP_VIEWPORT_EDGE, 0.0),
        Vec3::new(LEFT_VIEWPORT_EDGE, BOTTOM_VIEWPORT_EDGE, 0.0),
        // Left and right edges.
        Vec3::new(LEFT_VIEWPORT_EDGE, 0.0, 0.0),
        Vec3::new(RIGHT_VIEWPORT_EDGE, 0.0, 0.0),
        // Top edge.
        Vec3::new(LEFT_VIEWPORT_EDGE / 2., TOP_VIEWPORT_EDGE, 0.0),
        Vec3::new(RIGHT_VIEWPORT_EDGE / 2., TOP_VIEWPORT_EDGE, 0.0),
        // Bottom edge.
        Vec3::new(LEFT_VIEWPORT_EDGE / 2., BOTTOM_VIEWPORT_EDGE, 0.0),
        Vec3::new(RIGHT_VIEWPORT_EDGE / 2., BOTTOM_VIEWPORT_EDGE, 0.0),
    ];

    for spawner_point in spawner_points {
        let mut rng = thread_rng();
        let target: Vec3 = Vec3::new(
            rng.gen_range(-320.0..=320.0),
            rng.gen_range(-130.0..=130.0),
            0.0,
        );
        let direction = Transform::from_translation((target - spawner_point).normalize_or_zero());

        // Direction from A to B is B - A;
        // we then normalize it to make it a unit vector.
        commands.spawn(AsteroidSpawnerBundle::new(
            spawner_point,
            direction.translation,
        ));
    }
}

#[derive(Event, Debug)]
pub struct SpawnAsteroids {
    amount: u16,
}

impl SpawnAsteroids {
    pub fn new(amount: u16) -> Self {
        Self { amount }
    }
}

fn spawn_asteroids(
    trigger: Trigger<SpawnAsteroids>,
    query: Query<(&AsteroidSpawner, &Transform)>,
    mut commands: Commands,
    mut asteroid_id: ResMut<AsteroidID>,
) {
    let mut spawned_asteroids: Vec<Vec3> = vec![];
    let mut rng = rand::thread_rng();

    while spawned_asteroids.len() < trigger.event().amount as usize {
        let (spawner, transform) = query.iter().choose(&mut rng).unwrap();

        if spawned_asteroids.contains(&transform.translation) {
            continue;
        }

        let spawn_basic: bool = rng.gen();
        commands.trigger(SpawnAsteroid::new(
            if spawn_basic {AsteroidKind::Basic} else {AsteroidKind::Advanced},
            *transform,
            spawner.normal_direction,
            asteroid_id.get()
        ));

        spawned_asteroids.push(transform.translation);
    }
}

fn initial_spawn_asteroids(mut commands: Commands) {
    commands.trigger(SpawnAsteroids::new(5));
}

fn destroyed_asteroids(
    mut event_reader: EventReader<Destroyed>,
    asteroid_query: Query<(&Health, &Points, &AsteroidKind, &Transform, &Asteroid)>,
    mut score: ResMut<Score>,
    mut small_asteroid_map: ResMut<SmallAsteroidMap>,
    mut commands: Commands,
) {
    for Destroyed(entity) in event_reader.read() {
        if asteroid_query.contains(*entity) {
            let (health, points, kind, transform, asteroid) = asteroid_query.get(*entity).unwrap();
            if health.current() == 0 {
                score.current += points.0;
                if !kind.is_smaller() {
                    for n in -1..=1 {
                        const OFFSET: f32 = 40.;

                        let mut new_direction = Transform::from_translation(asteroid.direction);
                        new_direction.rotate_z(n as f32 * (PI / 8.));
                        let new_direction = new_direction.rotation.mul_vec3(new_direction.translation).normalize_or_zero();

                        let mut new_transform = *transform;
                        new_transform.translation += new_direction * OFFSET;
                        
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
