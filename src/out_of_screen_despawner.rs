
use bevy::prelude::*;
use crate::cow_tils::CowTag;
use crate::cow_nstants::CowNstants;

pub struct OutOfScreenDespawnerPlugin;

impl Plugin for OutOfScreenDespawnerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(update);
    }
}


fn setup(

){

}

fn update(
    mut commands: Commands,
    mut query: Query<Entity, With<CowTag>>,
    query2 : Query<&mut Transform>,
    cownstants : Res<CowNstants>,
){

    for entity in query.iter_mut() {
        let transform = *query2.get(entity).unwrap();

        let cow_pos = transform.translation;

        if cow_pos.x.is_nan() || cow_pos.y.is_nan() {
            if let Some(mut ent) = commands.get_entity(entity){
                ent.despawn();
            }
        }

        let bound_x : f32 = cownstants.window_size.x / 2.0;
        let bound_y : f32 = cownstants.window_size.y / 2.0;
        if cow_pos.x < -bound_x || cow_pos.x > bound_x || cow_pos.y < -bound_y || cow_pos.y > bound_y {
            if let Some(mut ent) = commands.get_entity(entity){
                ent.despawn();
            }
        }
    }

}