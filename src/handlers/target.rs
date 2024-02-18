use bevy::{math::bounding::{Aabb2d, IntersectsVolume}, prelude::*, window::PrimaryWindow};

use crate::utils::components::*;

pub fn target_handler(
    mut targetable_query: Query<(&mut Targetable, &GlobalTransform, &Sprite)>,
    buttons: Res<ButtonInput<MouseButton>>,
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
            let mut mouse_pos = Vec2::default();

            let (camera, camera_transform) = camera_query.single();

            if let Some(position) = window_query
                .single()
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate())
            {
                mouse_pos = position;
            };

            let mut sprite_size = Vec2::new(10.0, 10.0);

            if let Some(size) = sprite.custom_size {
                sprite_size = size + targetable.offset
            }

            let target_pos = Vec3::from(transform.affine().translation);

            let collision = Aabb2d::new(Vec2::from((target_pos.x, target_pos.y)), sprite_size / 2.)
            .intersects(&Aabb2d::new(mouse_pos, Vec2::new(2., 2.) / 2.));

            // let bounding = collide(target_pos, sprite_size, mouse_pos, Vec2::new(2., 2.));

            if collision {
                targetable.is_targeted = true;
                targetable.new_target = true;
                *reset_targets = true;
            }
        }
    }
}