use std::cmp;

use bevy::prelude::*;

use crate::{destruction::Destroyed, score::Score};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, zero_health);
    app.register_type::<LinearAcceleration>();
    app.register_type::<AngularAcceleration>();
    app.register_type::<Health>();
    app.register_type::<Points>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LinearAcceleration(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AngularAcceleration(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Health {
    max: u16,
    current: u16,
}

impl Health {
    pub fn new(max: u16) -> Self {
        Self { max, current: max }
    }

    pub fn add(&mut self, health: u16) {
        self.current = self.current.saturating_add(health);
        self.current = cmp::min(self.current, self.max);
    }

    pub fn sub(&mut self, health: u16) {
        self.current = self.current.saturating_sub(health);
    }

    pub fn current(&self) -> u16 {
        self.current
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Points(pub u32);

fn zero_health(
    mut destroyed_event_writer: EventWriter<Destroyed>,
    query: Query<(Entity, &Health, &Points), Changed<Health>>,
    mut score: ResMut<Score>,
) {
    for (entity, health, points) in &query {
        if health.current() == 0 {
            score.current += points.0;
            destroyed_event_writer.send(Destroyed(entity));
        }
    }
}
