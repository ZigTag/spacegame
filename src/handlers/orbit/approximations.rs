use bevy::prelude::*;

use crate::utils::{components::*, orbit};

pub fn move_objects(
    mut planet_children_query: Query<(&mut Transform, &OrbitInfo), Without<Sun>>,
    planet_query: Query<(&Children, &OrbitInfo)>,
    timer: Res<Time>,
) {
    for (children, origin_planet) in &planet_query {
        // let tick_time = self.sim_time as f32 / TICKS_PER_SECOND as f32;

        for &child in children.iter() {
            let (mut transform, planet) = planet_children_query.get_mut(child).unwrap();

            let orbital_position =
                orbit::approximations::calculate_orbital_position(origin_planet, planet, &timer.elapsed_seconds());

            transform.translation = orbital_position;
        }
    }
}