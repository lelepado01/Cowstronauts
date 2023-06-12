
use bevy::prelude::*;
use crate::cow_tils::MilkTag;
use crate::cow_tils::MilkMomentum;

pub struct MilkMoverPlugin;

impl Plugin for MilkMoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup)
            .add_system(update);
    }
}

fn setup(

){
    
}

fn update(
    mut query: Query<(&MilkTag, &mut Transform, &MilkMomentum)>,
    time : Res<Time>,
){

    for (_, mut transform, momentum) in query.iter_mut() {
        transform.translation += momentum.velocity * time.delta_seconds() * 100.0;
        transform.rotation *= momentum.rotation * Quat::from_rotation_z(time.delta_seconds() * 0.1) * time.delta_seconds();
    }

}

