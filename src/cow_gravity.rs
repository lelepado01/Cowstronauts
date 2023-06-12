
use crate::cow_quadtree::CowQuadTree;
use crate::cow_quadtree::quadtree_node::EntityData;
use crate::cow_tils::{CowTag, CowMomentum};
use crate::cow_nstants::CowNstants;
use bevy::prelude::*;

pub struct CowGravityPlugin;

impl Plugin for CowGravityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(update);
    }
}

fn setup(
    // mut commands: Commands,
){

}

fn update(
    cownstants : Res<CowNstants>,
    mut octree : ResMut<CowQuadTree>,
    mut query: Query<(Entity, &mut Transform, &mut CowMomentum), With<CowTag>>,
){

    let mut combinations = query.iter_combinations_mut();

    while let Some([(e1, mut transform1, mut momentum1), (e2, mut transform2, mut momentum2)]) = combinations.fetch_next() {

        let cow1_pos = transform1.translation;
        let cow2_pos = transform2.translation;

        let distance = (cow1_pos - cow2_pos).length();

        let mass1 = transform1.scale.x;
        let mass2 = transform2.scale.x;

        if distance < cownstants.max_attraction_distance {
            let gravity_force = cownstants.COWttraction_force * (mass1 * mass2) / distance;
            let directed_gravity_force = (cow1_pos - cow2_pos).normalize() * gravity_force;
            
            momentum1.velocity += directed_gravity_force / mass1;
            momentum2.velocity -= directed_gravity_force / mass2;
        }

        if momentum1.velocity.length() > cownstants.max_velocity {
            momentum1.velocity = momentum1.velocity.normalize() * cownstants.max_velocity;
        }
        if momentum2.velocity.length() > cownstants.max_velocity {
            momentum2.velocity = momentum2.velocity.normalize() * cownstants.max_velocity;
        }

        transform1.translation += momentum1.velocity;
        transform2.translation += momentum2.velocity;

        octree.update(EntityData{
            position: transform1.translation.truncate(),
            scale: cownstants.cow_size,
            entity: e1,
        });

        octree.update(EntityData{
            position: transform2.translation.truncate(),
            scale: cownstants.cow_size,
            entity: e2,
        });
    }

}