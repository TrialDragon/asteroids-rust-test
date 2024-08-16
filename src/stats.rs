use std::cmp;

pub use bevy::prelude::*;

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
    current:u16
}

impl Health {
    pub fn new(max: u16) -> Self {
        Self {
            max,
            current: 0,
        }
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


