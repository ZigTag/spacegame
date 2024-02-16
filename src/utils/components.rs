use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera {}

#[derive(Component)]
pub struct Targetable {
    pub new_target: bool,
    pub is_targeted: bool,
    pub offset: f32,
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
pub struct Sun {}

#[derive(Component)]
pub struct OrbitInfo {
    pub mass: f32,
    pub sma: f32,
    pub eccentricity: f32,
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
pub struct PlanetBundle {
    pub planet: OrbitInfo,
    pub sprite: SpriteBundle,
    pub targetable: Targetable,
}
