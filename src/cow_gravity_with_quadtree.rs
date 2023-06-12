
use crate::cow_quadtree::CowQuadTree;
use crate::cow_quadtree::aabb::AABB;
use crate::cow_quadtree::quadtree_node::EntityData;
use crate::cow_tils::{CowTag, CowMomentum};
use crate::cow_nstants::CowNstants;
use bevy::prelude::*;

pub struct CowGravityWithQuadtreePlugin;

impl Plugin for CowGravityWithQuadtreePlugin {
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

const cow_size : f32 = 500.0;

fn create_cow_bounds(cow_pos : Vec2) -> AABB {
    AABB::new(
        cow_pos - Vec2::new(cow_size, cow_size),
        cow_pos + Vec2::new(cow_size, cow_size),
    )
}

fn update(
    cownstants : Res<CowNstants>,
    mut octree : ResMut<CowQuadTree>,
    mut query: Query<(Entity, &mut Transform, &mut CowMomentum), With<CowTag>>,
){

    for (e, mut transform, mut momentum) in query.iter_mut() {

        let cow_bounds = create_cow_bounds(transform.translation.truncate());

        let positions = octree.query(cow_bounds);

        for (position, mass) in positions {
            let distance = (position - transform.translation.truncate()).length();

            let mass1 = transform.scale.x * transform.scale.x;
            let mass2 = mass * mass;


            if distance < cownstants.max_attraction_distance {
                let gravity_force = cownstants.COWttraction_force * (mass1 * mass2) / distance;
                let directed_gravity_force = (position - transform.translation.truncate()).normalize() * gravity_force;
                
                momentum.velocity += (directed_gravity_force / mass1).extend(0.0);
            }
        }

        if momentum.velocity.length() > cownstants.max_velocity {
            momentum.velocity = momentum.velocity.normalize() * cownstants.max_velocity;
        }

        transform.translation += momentum.velocity;

        octree.update(EntityData{
            position: transform.translation.truncate(),
            scale: cownstants.cow_size,
            entity: e,
        });
    }
}