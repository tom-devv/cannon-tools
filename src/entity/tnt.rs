use std::{
    arch::x86_64::_MM_FROUND_RAISE_EXC,
    f32::consts::FRAC_1_PI,
    ops::{Add, Mul},
};

use crate::{DRAG, FRICTION, GRAVITY, math::vector3::Vector3};

static EXPLOSION_HEIGHT: Vector3 = Vector3 {
    x: 0.0,
    y: (0.98 as f32 * 0.0625) as f64,
    z: 0.0,
};

#[derive(Debug, Clone)]
pub struct TNT {
    pub pos: Vector3,
    pub velocity: Vector3,
    pub explosion_position: Vector3,
    pub count: u64,
}

impl TNT {
    pub fn new(pos: Vector3, velocity: Vector3, count: u64) -> Self {
        TNT {
            pos,
            velocity,
            explosion_position: pos.add(EXPLOSION_HEIGHT),
            count,
        }
    }

    pub fn tick(&mut self) -> &mut Self {
        self.velocity.y -= GRAVITY;

        self.pos = self.pos.add(self.velocity);
        self.velocity = self.velocity.mul(DRAG);
        // self.velocity = self
        //     .velocity
        //     .component_mul(&Vector3::new(FRICTION, 1.0, FRICTION));

        // self.velocity = if ground {
        //     self.velocity
        //         .component_mul(&Vector3::new(FRICTION, 1.0, FRICTION));
        // } else {
        //     self.velocity
        // };

        self
    }
}
