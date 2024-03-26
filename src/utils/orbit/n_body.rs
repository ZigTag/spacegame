use bevy::prelude::*;

use crate::utils::{components::*, consts::G, orbit};

pub fn calculate_object_force(
    body_position: &Vec3,
    planet_position: &Vec3,
    body_mass: f32,
    planet_mass: f32,
) -> Vec3 {
    let pos_vector = *body_position - *planet_position;

    // r_hat
    let pos_mag = (pos_vector.x.powi(2) + pos_vector.y.powi(2) + pos_vector.z.powi(2)).sqrt();
    let pos_hat = pos_vector / pos_mag;

    let force_mag = (G * body_mass * planet_mass) / pos_mag.powi(2);

    // println!("{}", force_mag);

    force_mag * -pos_hat
}

pub fn calculate_prediction_forces(
    body_position: &Vec3,
    body_info: &NBody,
    planet_hash_map: &PlanetHashMap,
) -> Vec3 {
    let mut total_force = Vec3::default();

    for (_, planet_position, planet_info, _, _) in planet_hash_map.values() {
        if planet_info.mass <= 0.0 {
            continue;
        }

        // println!("{}", planet_info.mass);

        let force_vec = calculate_object_force(
            body_position,
            planet_position,
            body_info.mass,
            planet_info.mass,
        );

        // println!("{}: {} : {}", force_vec, body_position, planet_position);

        total_force += force_vec;
    }

    total_force
}

pub fn calculate_object_forces(
    body_position: &Vec3,
    body_info: &NBody,
    planet_query: &Query<(&GlobalTransform, &OrbitInfo, Option<&ReferenceFrame>), Without<NBody>>,
) -> (Vec3, Vec3) {
    let mut total_force = Vec3::default();
    let mut ref_pos = Vec3::default();

    for (planet_position, planet_info, reference_frame) in planet_query {
        if planet_info.mass <= 0.0 {
            continue;
        }

        let planet_position = Vec3::from(planet_position.affine().translation);

        if reference_frame.is_some() {
            ref_pos = planet_position
        }

        let force_vec = calculate_object_force(
            body_position,
            &planet_position,
            body_info.mass,
            planet_info.mass,
        );

        // println!("{}: {} : {}", force_vec, body_position, planet_position);

        total_force += force_vec;

        // There's no way its this easy
        // It really was that easy
    }

    (total_force, ref_pos)
}

pub fn relative_to_absolute(
    children: Vec<Entity>,
    parent_pos: Vec3,
    parent_planet: OrbitInfo,
    hash_on_my_map: &mut PlanetHashMap,
    temp_time: f32,
) -> Vec3 {
    let mut ref_pos = Vec3::default();

    for child in children {
        // println!("{}", hash_on_my_map[&child].1);

        let (child_children, position, planet_info, _, reference_frame) =
            hash_on_my_map[&child].clone();

        if reference_frame {
            ref_pos = position
        }

        let position = parent_pos
            + orbit::approximations::calculate_orbital_position(
                &parent_planet,
                &planet_info,
                &temp_time,
            );

        hash_on_my_map.get_mut(&child).unwrap().1 = position;

        // println!("{}", adjusted_pos);

        let returned_ref_pos = relative_to_absolute(
            child_children.clone(),
            position,
            planet_info.clone(),
            hash_on_my_map,
            temp_time,
        );

        if returned_ref_pos != Vec3::default() {
            ref_pos = returned_ref_pos;
        }
    }

    ref_pos
}
