use std::f32::consts::PI;

use bevy::{ecs::system::Res, math::Vec3, time::Time};

use crate::utils::consts::G;
use crate::OrbitInfo;

pub fn calculate_orbital_position(
    origin_planet: &OrbitInfo,
    planet: &OrbitInfo,
    timer: &Res<Time>,
) -> Vec3 {
    let eccentricity: f32 = planet.eccentricity;

    // ((transform.translation.x -u 0.).powi(2) + (transform.translation.y - 0.).powi(2)).sqrt();

    let gravitational_parameter = G * origin_planet.mass;

    let semimajor_axis: f32 = planet.sma;

    // let velocity = gravitational_parameter / planetary_distance.powi(2);

    let period = 2.0 * PI * (semimajor_axis.powi(3) / gravitational_parameter).sqrt();

    // let time = (2.0 * PI) * (semimajor_axis.powi(3) / gravitational_parameter).sqrt();

    // let sweep = 2.0 * PI / time;

    let mean_motion = 2. * PI / period;

    let mean_anomaly = mean_motion * timer.elapsed_seconds();

    let mut eccentric_anomaly: f32 = mean_anomaly;

    if eccentricity >= 0.8 {
        eccentric_anomaly = PI;
    }

    let mut pseudo_true_anomaly =
        eccentric_anomaly - eccentricity * mean_anomaly.sin() - mean_anomaly;

    let delta = 10e-8;
    let mut i = 0;
    let i_cap = 100;

    while (pseudo_true_anomaly.abs() > delta) && (i < i_cap) {
        eccentric_anomaly = eccentric_anomaly
            - pseudo_true_anomaly / (1. - (eccentricity * eccentric_anomaly.cos()));
        pseudo_true_anomaly =
            eccentric_anomaly - eccentricity * eccentric_anomaly.sin() - mean_anomaly;
        i += 1;
    }

    let true_anomaly = ((1. - eccentricity.powi(2)).sqrt() * eccentric_anomaly.sin())
        .atan2(eccentric_anomaly.cos() - eccentricity);

    let planetary_distance = semimajor_axis * (1. - (eccentricity * eccentric_anomaly.cos()));

    return planetary_distance * Vec3::new(true_anomaly.cos(), true_anomaly.sin(), 0.);

    // let argument_of_periapsis =
}

