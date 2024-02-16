use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    window::PrimaryWindow,
};

mod utils;

use utils::{camera::camera_handler, components::*, consts::G, orbit};

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
                transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
                ..default()
            },
            n_body: NBody {
                mass: 10.0,
                velocity: Vec3::from((0., 100., 0.)),
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

            let orbital_position = orbit::calculate_orbital_position(origin_planet, planet, &timer);

            transform.translation = orbital_position;
        }
    }
}

fn n_body_real(
    mut body_query: Query<(&mut Transform, &mut NBody), Without<OrbitInfo>>,
    planet_query: Query<(&Transform, &OrbitInfo), Without<NBody>>,
    timer: Res<Time>,
) {
    for (mut body_position, mut body_info) in &mut body_query {
        if body_info.mass <= 0.0 {
            continue;
        }

        let mut total_force = Vec3::default();

        let body_mass = body_info.mass;

        for (planet_position, planet_info) in &planet_query {
            if planet_info.mass <= 0.0 {
                continue;
            }

            let pos_vector = body_position.translation - planet_position.translation;

            // r_hat
            let pos_mag =
                (pos_vector.x.powi(2) + pos_vector.y.powi(2) + pos_vector.z.powi(2)).sqrt();
            let pos_hat = pos_vector / pos_mag;

            let force_mag = (G * body_info.mass * planet_info.mass) / pos_mag.powi(2);

            total_force += force_mag * -pos_hat;

            // There's no way its this easy
        }

        body_info.velocity += (total_force / body_mass) * timer.delta_seconds();

        body_position.translation += body_info.velocity * timer.delta_seconds();
    }
}
