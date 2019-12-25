use std::thread;
use std::sync::mpsc;

use itertools::Itertools;
use num::Integer;

// Serializer function
pub fn input_generator(moon_positions: String) -> Vec<Moon> {
    moon_positions.lines().map(|moon| Moon {
        position: [
            moon.split("x=").last().unwrap().split(',').nth(0).unwrap().parse().unwrap(),
            moon.split("y=").last().unwrap().split(',').nth(0).unwrap().parse().unwrap(),
            moon.split("z=").last().unwrap().split('>').nth(0).unwrap().parse().unwrap(),
        ],
        velocity: [0; 3],
    }).collect()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Moon {
    position: [i32; 3],
    velocity: [i32; 3],
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoonAxis {
    position: i32,
    velocity: i32,
}

fn step(moons: &mut Vec<Moon>) {
    for moon_pair in (0..4).combinations(2) {
        let moon_1 = moon_pair[0];
        let moon_2 = moon_pair[1];
        if moons[moon_1].position[0] > moons[moon_2].position[0] {
            moons[moon_1].velocity[0] -= 1;
            moons[moon_2].velocity[0] += 1;
        } else if moons[moon_1].position[0] < moons[moon_2].position[0] {
            moons[moon_1].velocity[0] += 1;
            moons[moon_2].velocity[0] -= 1;
        }
        if moons[moon_1].position[1] > moons[moon_2].position[1] {
            moons[moon_1].velocity[1] -= 1;
            moons[moon_2].velocity[1] += 1;
        } else if moons[moon_1].position[1] < moons[moon_2].position[1] {
            moons[moon_1].velocity[1] += 1;
            moons[moon_2].velocity[1] -= 1;
        }
        if moons[moon_1].position[2] > moons[moon_2].position[2] {
            moons[moon_1].velocity[2] -= 1;
            moons[moon_2].velocity[2] += 1;
        } else if moons[moon_1].position[2] < moons[moon_2].position[2] {
            moons[moon_1].velocity[2] += 1;
            moons[moon_2].velocity[2] -= 1;
        }
    }
    for moon in moons.iter_mut() {
        moon.position[0] += moon.velocity[0];
        moon.position[1] += moon.velocity[1];
        moon.position[2] += moon.velocity[2];
    }
}

fn step_axis(moons: &mut Vec<MoonAxis>) {
    for moon_pair in (0..4).combinations(2) {
        let moon_1 = moon_pair[0];
        let moon_2 = moon_pair[1];
        if moons[moon_1].position > moons[moon_2].position {
            moons[moon_1].velocity -= 1;
            moons[moon_2].velocity += 1;
        } else if moons[moon_1].position < moons[moon_2].position {
            moons[moon_1].velocity += 1;
            moons[moon_2].velocity -= 1;
        }
    }
    for i in 0..moons.len() {
        moons[i].position += moons[i].velocity;
    }
}

// Solver function for part 1
pub fn total_energy(moons: Vec<Moon>) -> i32 {
    let mut moons = moons.to_vec();
    for _ in 0..1000 {
        step(&mut moons);
    }
    moons.iter().map(|moon| {
        ((moon.position[0] as f32).abs() + (moon.position[1] as f32).abs()
        + (moon.position[2] as f32).abs()) * ((moon.velocity[0] as f32).abs()
        + (moon.velocity[1] as f32).abs() + (moon.velocity[2] as f32).abs())
    }).sum::<f32>() as i32
}

// Solver function for part 2
pub fn steps_before_repeating(moons: Vec<Moon>) -> i64 {
    let mut moons_x: Vec<MoonAxis> = moons.iter().map(|moon| MoonAxis {
        position: moon.position[0].clone(),
        velocity: moon.velocity[0].clone(),
    }).collect();
    let mut moons_y: Vec<MoonAxis> = moons.iter().map(|moon| MoonAxis {
        position: moon.position[1].clone(),
        velocity: moon.velocity[1].clone(),
    }).collect();
    let mut moons_z: Vec<MoonAxis> = moons.iter().map(|moon| MoonAxis {
        position: moon.position[2].clone(),
        velocity: moon.velocity[2].clone(),
    }).collect();
    let initial_x = moons_x.clone();
    let initial_y = moons_y.clone();
    let initial_z = moons_z.clone();

    let (xtx, xrx) = mpsc::channel();
    let (ytx, yrx) = mpsc::channel();
    let (ztx, zrx) = mpsc::channel();

    let x = thread::spawn(move || {
        let mut i: i64 = 1;
        step_axis(&mut moons_x);
        while moons_x != initial_x {
            i += 1;
            step_axis(&mut moons_x);
        }
        xtx.send(i).unwrap();
    });

    let y = thread::spawn(move || {
        let mut i: i64 = 1;
        step_axis(&mut moons_y);
        while moons_y != initial_y {
            i += 1;
            step_axis(&mut moons_y);
        }
        ytx.send(i).unwrap();
    });

    let z = thread::spawn(move || {
        let mut i: i64 = 1;
        step_axis(&mut moons_z);
        while moons_z != initial_z {
            i += 1;
            step_axis(&mut moons_z);
        }
        ztx.send(i).unwrap();
    });

    let x_step = xrx.recv().unwrap();
    let y_step = yrx.recv().unwrap();
    let z_step = zrx.recv().unwrap();

    x.join().unwrap();
    y.join().unwrap();
    z.join().unwrap();

    z_step.lcm(&y_step.lcm(&x_step))
}
