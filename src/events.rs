use bevy::prelude::*;


#[derive(Event, Debug)]
pub struct TorpedoCollisionEvent {
    pub position: Vec3,
    pub damage: u8,
    pub radius: f32,
    pub torpedo: Entity
}

