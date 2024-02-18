use bevy::prelude::*;

use crate::utils::{components::*, orbit};

pub fn n_body_computation(
    mut body_query: Query<(&mut Transform, &mut NBody), Without<OrbitInfo>>,
    planet_query: Query<(&GlobalTransform, &OrbitInfo), Without<NBody>>,
    shadow_planet_query: Query<(
        Entity,
        Option<&Children>,
        &Transform,
        &OrbitInfo,
        Option<&Sun>,
    )>,
    timer: Res<Time>,
    mut gizmos: Gizmos,
    mut local_planet_hash_map: Local<PlanetHashMap>,
    mut sun_key: Local<Option<Entity>>,
) {
    for (mut body_position, mut body_info) in &mut body_query {
        if body_info.mass <= 0.0 {
            continue;
        }

        let body_mass = body_info.mass;

        let total_force =
            orbit::n_body::calculate_object_forces(&body_position.translation, &body_info, &planet_query);

        body_info.velocity += (total_force / body_mass) * timer.delta_seconds();

        body_position.translation += body_info.velocity * timer.delta_seconds();

        if body_info.prediction.show {
            if local_planet_hash_map.keys().len() == 0 {
                for (entity_id, children, transform, orbit_info, sun) in &shadow_planet_query {
                    let mut entity_children = vec![];

                    if let Some(children) = children {
                        for child in children {
                            entity_children.push(*child);
                        }
                    }

                    if sun.is_some() {
                        *sun_key = Some(entity_id);
                    }

                    local_planet_hash_map.insert(
                        entity_id,
                        (
                            entity_children,
                            transform.translation,
                            orbit_info.clone(),
                            sun.is_some(),
                        ),
                    );
                }
            }

            // println!("{:?}", when_she_hash_on_my_map);

            let mut temp_velocity = body_info.velocity;
            let mut temp_position = body_position.translation;
            let mut temp_time = timer.elapsed_seconds();

            let time_step = body_info.prediction.time / body_info.prediction.segments as f32;

            let mut segments: Vec<Vec2> = vec![];

            // let mut orbit_lines: HashMap<Entity, Vec<Vec2>> = HashMap::new();

            for _segment in 0..body_info.prediction.segments {
                // for key in when_she_hash_on_my_map.clone().keys() {
                //     let origin_planet = when_she_hash_on_my_map[key].2.clone();

                //     for key2 in &when_she_hash_on_my_map[key].0.clone() {
                //         let child_planet = when_she_hash_on_my_map[key2].2.clone();

                //         when_she_hash_on_my_map.get_mut(key).unwrap().1 =

                //         // println!("{:?}", when_she_hash_on_my_map);
                //     }
                // }

                if let Some(sun_key) = *sun_key {
                    let sun_items = local_planet_hash_map[&sun_key].clone();

                    orbit::n_body::relative_to_absolute(
                        sun_items.0.clone(),
                        sun_items.1,
                        sun_items.2,
                        &mut local_planet_hash_map,
                        temp_time,
                    )
                }

                let temp_force = orbit::n_body::calculate_prediction_forces(
                    &temp_position,
                    &body_info,
                    &local_planet_hash_map,
                );

                // if _segment == 0 {
                //     println!("{:?}", temp_force);
                // }

                segments.push(Vec2::from((temp_position.x, temp_position.y)));

                // Does Orbit Lines, fully functional.

                // for (i, j) in &when_she_hash_on_my_map {
                //     if !orbit_lines.contains_key(i) {
                //         orbit_lines.insert(*i, vec![]);
                //     }

                //     orbit_lines
                //         .get_mut(i)
                //         .unwrap()
                //         .push(Vec2::from((j.1.x, j.1.y)));
                // }

                temp_velocity += (temp_force / body_mass) * time_step;
                temp_position += temp_velocity * time_step;

                temp_time += time_step;
            }

            // for (_, j) in orbit_lines {
            //     gizmos.linestrip_2d(j, Color::TEAL)
            // }

            gizmos.linestrip_2d(segments, Color::CYAN);
        }
    }
}
