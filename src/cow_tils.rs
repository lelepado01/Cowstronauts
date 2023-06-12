
use bevy::prelude::*;

#[derive(Component)]
pub struct CowTag;

#[derive(Component)]
pub struct CowMomentum{
    pub velocity: Vec3,
}