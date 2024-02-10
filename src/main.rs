use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    window::PrimaryWindow,
};

mod utils;

use utils::orbit;

#[derive(Component)]
struct MainCamera {}

#[derive(Component)]
struct Targetable {
    new_target: bool,
    is_targeted: bool,
    offset: f32,
}

impl Default for Targetable {
    fn default() -> Self {
        Targetable {
            new_target: false,
            is_targeted: false,
            offset: 10.0,
        }
    }
}

#[derive(Component)]
struct Sun {}

#[derive(Component)]
struct OrbitInfo {
    mass: f32,
    sma: f32,
    eccentricity: f32,
}

impl Default for OrbitInfo {
    fn default() -> Self {
        OrbitInfo {
            mass: 1.0,
            sma: 1.0,
            eccentricity: 0.0,
        }
    }
}

#[derive(Bundle, Default)]
struct PlanetBundle {
    planet: OrbitInfo,
    sprite: SpriteBundle,
    targetable: Targetable,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_objects.before(camera_handler),
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

    commands.entity(satellite).push_children(&[moon]);

    commands.entity(sun).push_children(&[satellite]);

    commands.entity(world).push_children(&[sun]);
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

fn camera_handler(
    mut camera_query: Query<(&mut Transform, &Camera), With<MainCamera>>,
    target_query: Query<(&Parent, &GlobalTransform, &Targetable), Without<Camera>>,
    parent_query: Query<&GlobalTransform, Without<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    timer: Res<Time>,
) {
    for (mut camera_transform, camera) in &mut camera_query {
        let mut target_pos = Vec3::default();
        let mut mouse_pos = Vec3::default();
        let mut parent_pos = Vec3::default();

        for (parent, target_transform, targetable) in &target_query {
            if targetable.is_targeted {
                let parent_transform = parent_query.get(parent.get()).unwrap();

                parent_pos = Vec3::from(parent_transform.affine().translation);

                target_pos = Vec3::from(target_transform.affine().translation);
            }
        }

        if let Some(position) = window_query.single().cursor_position() {
            let cam_size = camera.logical_viewport_size().unwrap();

            let mut new_pos = (cam_size / 2.) - position;

            new_pos.x = -new_pos.x;

            mouse_pos = Vec3::from((new_pos, 0.));
        };

        let camera_speed = 4.0;
        let horiz_speed = 0.3;
        let vert_speed = 0.3;

        let x_movement = horiz_speed * mouse_pos.x;
        let y_movement = vert_speed * mouse_pos.y;

        let calculated_mouse_pos = Vec3::new(x_movement, y_movement, 0.);

        let final_pos = calculated_mouse_pos + parent_pos.lerp(target_pos, 0.45);

        let cam_lerp = camera_transform
            .translation
            .lerp(final_pos, camera_speed * timer.delta_seconds());

        camera_transform.translation = cam_lerp;
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
