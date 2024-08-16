use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_event::<Destroyed>();
}

#[derive(Event,Debug)]
pub struct Destroyed(pub Entity);
