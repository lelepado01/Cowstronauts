#![allow(non_snake_case)]

use bevy::prelude::*;

#[derive(Resource)]
pub struct CowNstants{
    pub window_size : Vec2,
    
    pub ui_position : Vec2,
    pub ui_size : Vec2,
    
    pub max_velocity : f32,
    // pub repel_force : f32,
    pub COWttraction_force : f32,
    pub cow_size : f32,

    pub max_attraction_distance : f32,
    pub max_repulsion_distance : f32,
}