use std::{
    fs::File,
    io::{BufWriter, Write},
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Instant,
};

use std::thread;

use itertools::iproduct;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    entity::{physics::get_motion, tnt::TNT},
    math::vector3::Vector3,
};

pub fn insane(projectile: &mut TNT) {
    let start = Instant::now(); // Start timer
    let range = 0..50;
    let total = (range.end - range.start) as usize;
    let total = total.pow(6);

    let counter = Arc::new(AtomicUsize::new(0));
    let last_percent = Arc::new(AtomicUsize::new(0));

    iproduct!(
        range.clone(),
        range.clone(),
        range.clone(),
        range.clone(),
        range.clone(),
        range.clone()
    )
    .par_bridge()
    .for_each(|comb| {
        let mut powers = get_powers();
        let mut copy = projectile.clone();
        for (power, value) in powers.iter_mut().zip([comb.0, comb.1, comb.2, comb.3]) {
            power.count = value as u64;
            get_motion(power, &mut copy);
        }

        copy.tick();

        let small = is_twelve_exact(&copy);
        if small {
            println!("{:?}", copy.pos.y);
        }

        let count = counter.fetch_add(1, Ordering::Relaxed);
        let percent = count * 100 / total;

        // Only print if percent increased since last print
        let mut last = last_percent.load(Ordering::Relaxed);
        while percent > last {
            if last_percent
                .compare_exchange(last, percent, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                println!("Progress: {}%", percent);
                break;
            } else {
                last = last_percent.load(Ordering::Relaxed);
            }
        }
    });
    let duration = start.elapsed(); // Calculate elapsed time
    println!("Execution took: {:.2?}", duration);
}

fn get_powers() -> Vec<TNT> {
    let powers = vec![
        TNT::new(
            Vector3::new(0.0, 240.32443734061044, 0.0),
            Vector3::zero(),
            31,
        ),
        TNT::new(
            Vector3::new(0.0, 240.32443734061044, 1.0),
            Vector3::zero(),
            22,
        ),
        TNT::new(
            Vector3::new(0.0, 240.32443734061044, 2.0),
            Vector3::zero(),
            20,
        ),
        TNT::new(
            Vector3::new(0.0, 240.32443734061044, 3.0),
            Vector3::zero(),
            17,
        ),
        TNT::new(
            Vector3::new(0.0, 240.32443734061044, 4.0),
            Vector3::zero(),
            17,
        ),
        TNT::new(
            Vector3::new(0.0, 240.32443734061044, 5.0),
            Vector3::zero(),
            17,
        ),
    ];

    powers
}

fn is_twelve_exact(projectile: &TNT) -> bool {
    let frac = projectile.pos.y.abs().fract();
    frac < 1e-12
}
