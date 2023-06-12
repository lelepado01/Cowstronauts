
use std::ops::Add;

use bevy::{prelude::*};
use crate::cow_quadtree::CowQuadTree;
use crate::cow_tils::{CowTag, CowMomentum};
use crate::cow_nstants::CowNstants;
use crate::cow_ui::mouse_click_grabbed; 
use crate::cow_quadtree::quadtree_node::EntityData;

pub struct CowSpawnerPlugin;

impl Plugin for CowSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(update);
    }
}

#[derive(Resource)]
struct CowSpriteResource{
    cow_sprite: Handle<Image>,
}


fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
){
    let cow_texture = asset_server.load("cow.png");
    
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(CowSpriteResource{
        cow_sprite: cow_texture.clone(),
    });
}

fn update(
    mut commands: Commands,
    cow_sprite_resource: Res<CowSpriteResource>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut mouse_motion_events: EventReader<CursorMoved>,
    cownstants : Res<CowNstants>,
    mut octree : ResMut<CowQuadTree>,
){

    if mouse_button_input.pressed(MouseButton::Left) {
    
        for event in mouse_motion_events.iter() {

            let pos = event.position-Vec2::new(cownstants.window_size.x / 2.0, cownstants.window_size.y / 2.0); 

            let ui_pos = Vec2::new(event.position.x, cownstants.window_size.y - event.position.y); 
            if mouse_click_grabbed(ui_pos, cownstants.ui_position, cownstants.ui_size.add(Vec2::new(40.0, 40.0))) {
                continue;
            }

            let transform = Transform{
                translation: Vec3::new(pos.x, pos.y, 0.0),
                scale: Vec3::splat(cownstants.cow_size),
                ..default()
            }; 

            let entity = commands.spawn((SpriteBundle {
                    texture: cow_sprite_resource.cow_sprite.clone(),
                    transform: transform,
                    ..default()
                }, 
                CowTag,
                CowMomentum{
                    velocity: Vec3::new(0.0, 0.0, 0.0),
                }
            ));

            let cow : EntityData = EntityData {
                position: Vec2::new(pos.x, pos.y),
                scale: cownstants.cow_size,
                entity: entity.id(),
            };

            octree.insert(cow);
        }
    }
    
}