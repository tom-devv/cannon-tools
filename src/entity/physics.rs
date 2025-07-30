use std::{
    clone,
    ffi::c_float,
    ops::{Add, Div, Mul, Sub},
};

use crate::{GRAVITY, entity::tnt::TNT, math::vector3::Vector3};

pub fn get_motion(power: &TNT, projectile: &mut TNT) {
    let diff = projectile.pos - power.explosion_position;
    let magnitude = diff.length();

    if magnitude >= 8.0 {
        return;
    }

    let exposure: f64 = (1.0 - magnitude / 8.0);

    let normalized_direction = diff.div(magnitude);

    let scaled_direction = normalized_direction.mul(exposure);

    let delta_motion = scaled_direction.mul(power.count as f64);

    projectile.velocity = projectile.velocity.add(delta_motion);
}

pub fn swing(power: &mut TNT, projectile: &mut TNT) {
    for i in 0..power.count {
        let mut swing_pos = power.pos.add(power.velocity);
        swing_pos.y -= GRAVITY;

        let swing_tnt = TNT::new(swing_pos, Vector3::zero(), 1);

        get_motion(&swing_tnt, power);
        get_motion(&swing_tnt, projectile);
    }
}
