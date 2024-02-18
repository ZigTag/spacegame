use std::collections::HashMap;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    window::PrimaryWindow,
};

mod utils;

use utils::{
    camera::camera_handler,
    components::*,
    consts::G,
    orbit::{self, calculate_orbital_position},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_objects.before(camera_handler),
                n_body_real.before(camera_handler).after(move_objects),
                camera_handler,
                target_handler,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            ..default()
        },
        MainCamera {},
    ));

    let world = commands.spawn(TransformBundle::default()).id();

    // Rectangle
    let sun = commands
        .spawn((
            PlanetBundle {
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(1., 1., 1.),
                        custom_size: Some(Vec2::new(100.0, 100.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                    ..default()
                },
                planet: OrbitInfo {
                    mass: 1.0e16,
                    sma: 1.,
                    eccentricity: 0.,
                },
                targetable: Targetable {
                    is_targeted: true,
                    ..default()
                },
                ..default()
            },
            Sun {},
        ))
        .id();

    let satellite = commands
        .spawn(PlanetBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1., 1., 1.),
                    custom_size: Some(Vec2::new(25.0, 25.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(200., 0., 0.)),
                ..default()
            },
            planet: OrbitInfo {
                mass: 5.0e14,
                sma: 200.,
                eccentricity: 0.,
            },
            ..Default::default()
        })
        .id();

    let moon = commands
        .spawn(PlanetBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1., 1., 1.),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(200., 0., 0.)),
                ..default()
            },
            planet: OrbitInfo {
                mass: 50.,
                sma: 30.,
                eccentricity: 0.,
            },
            ..default()
        })
        .id();

    let n_body_object = commands
        .spawn(NBodyBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1., 1., 1.),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
                ..default()
            },
            n_body: NBody {
                mass: 100.0,
                velocity: Vec3::from((0., 100., 0.)),
                prediction: Prediction {
                    show: true,
                    ..default()
                },
            },
            ..default()
        })
        .id();

    commands.entity(satellite).push_children(&[moon]);

    commands.entity(sun).push_children(&[satellite]);

    commands.entity(world).push_children(&[sun, n_body_object]);
}

fn target_handler(
    mut targetable_query: Query<(&mut Targetable, &GlobalTransform, &Sprite)>,
    buttons: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut reset_targets: Local<bool>,
) {
    for (mut targetable, transform, sprite) in &mut targetable_query {
        if !targetable.new_target && *reset_targets {
            targetable.is_targeted = false;
            continue;
        } else if targetable.new_target && *reset_targets {
            *reset_targets = false;
            targetable.new_target = false;
            continue;
        }

        if buttons.just_pressed(MouseButton::Left) {
            let mut mouse_pos = Vec3::default();

            let (camera, camera_transform) = camera_query.single();

            if let Some(position) = window_query
                .single()
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate())
            {
                mouse_pos = Vec3::from((position, 0.));
            };

            let mut sprite_size = Vec2::new(10.0, 10.0);

            if let Some(size) = sprite.custom_size {
                sprite_size = size + targetable.offset
            }

            let target_pos = Vec3::from(transform.affine().translation);

            let bounding = collide(target_pos, sprite_size, mouse_pos, Vec2::new(2., 2.));

            if let Some(bounding) = bounding {
                if bounding == Collision::Inside {
                    targetable.is_targeted = true;
                    targetable.new_target = true;
                    *reset_targets = true;
                }
            }
        }
    }
}

fn move_objects(
    mut planet_children_query: Query<(&mut Transform, &OrbitInfo), Without<Sun>>,
    planet_query: Query<(&Children, &OrbitInfo)>,
    timer: Res<Time>,
) {
    for (children, origin_planet) in &planet_query {
        // let tick_time = self.sim_time as f32 / TICKS_PER_SECOND as f32;

        for &child in children.iter() {
            let (mut transform, planet) = planet_children_query.get_mut(child).unwrap();

            let orbital_position =
                orbit::calculate_orbital_position(origin_planet, planet, &timer.elapsed_seconds());

            transform.translation = orbital_position;
        }
    }
}

fn calculate_object_force(
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

    println!("{}", force_mag);

    force_mag * -pos_hat
}

fn calculate_prediction_forces(
    body_position: &Vec3,
    body_info: &NBody,
    when_she_hash_on_my_map: &WhenSheHash,
) -> Vec3 {
    let mut total_force = Vec3::default();

    for (_, planet_position, planet_info, _) in when_she_hash_on_my_map.values() {
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

fn calculate_object_forces(
    body_position: &Vec3,
    body_info: &NBody,
    planet_query: &Query<(&GlobalTransform, &OrbitInfo), Without<NBody>>,
) -> Vec3 {
    let mut total_force = Vec3::default();

    for (planet_position, planet_info) in planet_query {
        if planet_info.mass <= 0.0 {
            continue;
        }

        let planet_position = Vec3::from(planet_position.affine().translation);

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

    total_force
}

type WhenSheHash = HashMap<Entity, (Vec<Entity>, Vec3, OrbitInfo, bool)>;

fn when_she_relative_to_my_absolute(
    children: Vec<Entity>,
    parent_pos: Vec3,
    hash_on_my_map: &mut WhenSheHash,
) {
    for child in children {
        // println!("{}", hash_on_my_map[&child].1);

        let adjusted_pos = parent_pos + hash_on_my_map[&child].1;

        // println!("{}", adjusted_pos);

        hash_on_my_map.get_mut(&child).unwrap().1 = adjusted_pos;

        when_she_relative_to_my_absolute(
            hash_on_my_map[&child].0.clone(),
            adjusted_pos,
            hash_on_my_map,
        )
    }
}

fn n_body_real(
    mut body_query: Query<(&mut Transform, &mut NBody), Without<OrbitInfo>>,
    planet_query: Query<(&GlobalTransform, &OrbitInfo), Without<NBody>>,
    query_these_nuts: Query<(
        Entity,
        Option<&Children>,
        &Transform,
        &OrbitInfo,
        Option<&Sun>,
    )>,
    timer: Res<Time>,
    mut gizmos: Gizmos,
    mut when_she_hash_on_my_map: Local<WhenSheHash>,
    mut sun_key: Local<Option<Entity>>,
) {
    for (mut body_position, mut body_info) in &mut body_query {
        if body_info.mass <= 0.0 {
            continue;
        }

        let body_mass = body_info.mass;

        let total_force =
            calculate_object_forces(&body_position.translation, &body_info, &planet_query);

        body_info.velocity += (total_force / body_mass) * timer.delta_seconds();

        body_position.translation += body_info.velocity * timer.delta_seconds();

        if body_info.prediction.show {
            if when_she_hash_on_my_map.keys().len() == 0 {
                for (entity_id, children, transform, orbit_info, sun) in &query_these_nuts {
                    let mut entity_children = vec![];

                    if let Some(children) = children {
                        for child in children {
                            entity_children.push(*child);
                        }
                    }

                    if sun.is_some() {
                        *sun_key = Some(entity_id);
                    }

                    when_she_hash_on_my_map.insert(
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

            let mut orbit_lines: HashMap<Entity, Vec<Vec2>> = HashMap::new();

            for _segment in 0..body_info.prediction.segments {
                for key in when_she_hash_on_my_map.clone().keys() {
                    let origin_planet = when_she_hash_on_my_map[key].2.clone();

                    for key2 in &when_she_hash_on_my_map[key].0.clone() {
                        let child_planet = when_she_hash_on_my_map[key2].2.clone();

                        when_she_hash_on_my_map.get_mut(key).unwrap().1 =
                            calculate_orbital_position(&origin_planet, &child_planet, &temp_time);

                        // println!("{:?}", when_she_hash_on_my_map);
                    }
                }

                if let Some(sun_key) = *sun_key {
                    let sun_items = when_she_hash_on_my_map[&sun_key].clone();

                    when_she_relative_to_my_absolute(
                        sun_items.0.clone(),
                        sun_items.1,
                        &mut when_she_hash_on_my_map,
                    )
                }

                let temp_force = calculate_prediction_forces(
                    &temp_position,
                    &body_info,
                    &when_she_hash_on_my_map,
                );

                if _segment == 0 {
                    println!("{:?}", temp_force);
                }

                segments.push(Vec2::from((temp_position.x, temp_position.y)));

                for (i, j) in &when_she_hash_on_my_map {
                    if !orbit_lines.contains_key(i) {
                        orbit_lines.insert(*i, vec![]);
                    }

                    orbit_lines
                        .get_mut(i)
                        .unwrap()
                        .push(Vec2::from((j.1.x, j.1.y)));
                }

                temp_velocity += (temp_force / body_mass) * time_step;
                temp_position += temp_velocity * time_step;

                temp_time += time_step;
            }

            for (_, j) in orbit_lines {
                gizmos.linestrip_2d(j, Color::TEAL)
            }

            gizmos.linestrip_2d(segments, Color::CYAN);
        }
    }
}
