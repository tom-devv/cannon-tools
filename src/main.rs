use crate::{
    entity::{
        physics::{get_motion, swing},
        tnt::TNT,
    },
    math::vector3::Vector3,
    scripts::angel::insane,
};
use itertools::iproduct;
use rayon::prelude::*;
use std::{
    io::Error,
    ops::{Add, Mul},
};

mod entity;
mod math;
mod scripts;

pub const GRAVITY: f64 = 0.04;
pub const DRAG: f64 = 0.98;
pub const FRICTION: f64 = 0.7;

fn main() {
    let mut power = TNT::new(Vector3::new(0.0, -55.0, 0.0), Vector3::zero(), 1);

    let mut projectile = TNT::new(Vector3::new(1.0, -54.0, 0.0), Vector3::zero(), 1);

    get_motion(&power, &mut projectile);

    println!("{:?}", projectile.velocity);

    // projectile.tick();

    // println!("{:?}", projectile.velocity);
    // projectile.tick();

    // println!("{:?}", projectile.velocity);
    // projectile.tick();

    // println!("{:?}", projectile.velocity);
    // projectile.tick();

    // println!("{:?}", projectile.velocity);
    // projectile.tick();

    // println!("{:?}", projectile.velocity);
    // projectile.tick();

    // println!("{:?}", projectile.velocity);
}
