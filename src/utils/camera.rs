use bevy::{prelude::*, window::PrimaryWindow};

use crate::utils::components::{MainCamera, Targetable};

pub fn camera_handler(
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
