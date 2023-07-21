
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod cow_tils;
mod cow_spawner;
mod cow_gravity;
mod cow_gravity_with_quadtree;
mod out_of_screen_despawner;
mod cow_nstants;
mod cow_ui; 
mod cow_quadtree;
use crate::cow_spawner::CowSpawnerPlugin;
use crate::cow_gravity::CowGravityPlugin;
use crate::cow_gravity_with_quadtree::CowGravityWithQuadtreePlugin;
use crate::out_of_screen_despawner::OutOfScreenDespawnerPlugin;
use crate::cow_nstants::CowNstants;
use crate::cow_ui::CowUiPlugin;

fn main(){

    let window_size = 1000.0;
    let window_desc = WindowDescriptor {
        title: "CowStronauts".to_string(),
        width: window_size,
        height: window_size,
        resizable: false,
        ..default()
    }; 

    let octree = cow_quadtree::CowQuadTree::new(window_size);

    let mut group = DefaultPlugins.set(WindowPlugin {
        window: window_desc,
        ..Default::default()
    }); 
    group = group.set(ImagePlugin::default_nearest());

    let cownstants = CowNstants{
        max_velocity : 0.1,

        window_size : Vec2::new(window_size, window_size),

        ui_position : Vec2::new(0.0, 0.0),
        ui_size : Vec2::new(200.0, 200.0),

        COWttraction_force : -0.005,

        cow_size : 1.0,

        max_attraction_distance : 2000.0,
        max_repulsion_distance : 5.0,
    };

    App::default()
        .add_plugins(group)
        .add_plugin(EguiPlugin)
        .add_plugin(CowUiPlugin)
        .add_plugin(CowSpawnerPlugin)
        .add_plugin(CowGravityWithQuadtreePlugin)
        .add_plugin(CowGravityPlugin)
        // .add_plugin(OutOfScreenDespawnerPlugin)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.4)))
        .insert_resource(cownstants)
        .insert_resource(octree)
        .run();
}