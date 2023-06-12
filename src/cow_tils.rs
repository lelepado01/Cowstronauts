
use bevy::prelude::*;

#[derive(Component)]
pub struct CowTag;

#[derive(Component)]
pub struct CowMomentum{
    pub velocity: Vec3,
}

#[derive(Resource)]
pub struct MilkSpriteResource{
    pub milk_sprite: Handle<Image>,
}

#[derive(Component)]
pub struct MilkTag;

#[derive(Component)]
pub struct MilkMomentum {
    pub velocity : Vec3,
    pub rotation: Quat,
}