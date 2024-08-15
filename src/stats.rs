pub use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LinearAcceleration(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AngularAcceleration(pub f32);
