use bevy::prelude::*;

mod utils;
mod handlers;

use utils::{
    camera::camera_handler,
    components::*,
};

use handlers::{
    target::target_handler,
    orbit::approximations::move_objects,
    orbit::n_body::n_body_computation,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_objects.before(camera_handler),
                n_body_computation.before(camera_handler).after(move_objects),
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
            camera: Camera {
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
        .spawn((
            PlanetBundle {
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
            },
            ReferenceFrame {},
        ))
        .id();

    let moon = commands
        .spawn((PlanetBundle {
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
        },
        // ReferenceFrame {},
    ))
        .id();

    let n_body_object = commands
        .spawn(NBodyBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1., 1., 1.),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(180., -35., 0.)),
                ..default()
            },
            n_body: NBody {
                mass: 100.0,
                velocity: Vec3::from((-15., 80., 0.)),
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