use std::collections::HashMap;

use bevy::prelude::*;

pub struct Prediction {
    pub time: f32,
    pub segments: i32,
    pub show: bool,
}

impl Default for Prediction {
    fn default() -> Self {
        Prediction {
            time: 10.0,
            segments: 1024,
            show: false,
        }
    }
}

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

#[derive(Component, Clone, Debug)]
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

#[derive(Component)]
pub struct NBody {
    pub mass: f32,
    pub velocity: Vec3,
    pub prediction: Prediction,
}

impl Default for NBody {
    fn default() -> Self {
        NBody {
            mass: 10.0,
            velocity: Vec3::default(),
            prediction: Prediction::default(),
        }
    }
}

#[derive(Component)]
pub struct ReferenceFrame {}

#[derive(Bundle, Default)]
pub struct PlanetBundle {
    pub planet: OrbitInfo,
    pub sprite: SpriteBundle,
    pub targetable: Targetable,
}

#[derive(Bundle, Default)]
pub struct NBodyBundle {
    pub n_body: NBody,
    pub sprite: SpriteBundle,
    pub targetable: Targetable,
}

pub type PlanetHashMap = HashMap<Entity, (Vec<Entity>, Vec3, OrbitInfo, bool, bool)>;