use crate::intcode_computer::Computer;
use std::collections::{VecDeque, HashMap};
use itertools::Itertools;

enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

// Solver function for part 1
pub fn panels_painted_at_least_once(intcode: Vec<i64>) -> usize {
    let mut panels_painted = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut direction = Direction::Top;
    let mut emergency_hull_painting_robot = Computer {
        memory: intcode.to_vec(),
        inputs: VecDeque::from(vec![0]),
        ..Default::default()
    };
    while !emergency_hull_painting_robot.run() {
        let outputs_length = emergency_hull_painting_robot.outputs.len();
        panels_painted.insert((x, y), emergency_hull_painting_robot.outputs[outputs_length - 2]);
        match emergency_hull_painting_robot.outputs[outputs_length - 1] {
            0 => {
                match direction {
                    Direction::Top => {
                        direction = Direction::Left;
                        x -= 1;
                    }
                    Direction::Right => {
                        direction = Direction::Top;
                        y += 1;
                    }
                    Direction::Bottom => {
                        direction = Direction::Right;
                        x += 1;
                    }
                    Direction::Left => {
                        direction = Direction::Bottom;
                        y -= 1;
                    }
                }
            }
            1 => {
                match direction {
                    Direction::Top => {
                        direction = Direction::Right;
                        x += 1;
                    }
                    Direction::Right => {
                        direction = Direction::Bottom;
                        y -= 1;
                    }
                    Direction::Bottom => {
                        direction = Direction::Left;
                        x -= 1;
                    }
                    Direction::Left => {
                        direction = Direction::Top;
                        y += 1;
                    }
                }
            }
            _ => unreachable!()
        }
        emergency_hull_painting_robot.inputs.push_back(panels_painted.get(&(x, y)).unwrap_or(&0).clone());
    }
    panels_painted.len()
}

// Solver function for part 2
pub fn registration_identifier(intcode: Vec<i64>) -> String {
    let mut panels_painted = HashMap::new();
    let mut width = (0, 0);
    let mut height = (0, 0);
    let mut x = 0;
    let mut y = 0;
    let mut direction = Direction::Top;
    let mut emergency_hull_painting_robot = Computer {
        memory: intcode.to_vec(),
        inputs: VecDeque::from(vec![1]),
        ..Default::default()
    };
    while !emergency_hull_painting_robot.run() {
        let outputs_length = emergency_hull_painting_robot.outputs.len();
        panels_painted.insert((x, y), emergency_hull_painting_robot.outputs[outputs_length - 2]);
        if x > width.1 {
            width.1 = x;
        } else if x < width.0 {
            width.0 = x;
        }
        if y > height.1 {
            height.1 = y;
        } else if y < height.0 {
            height.0 = y;
        }
        match emergency_hull_painting_robot.outputs[outputs_length - 1] {
            0 => {
                match direction {
                    Direction::Top => {
                        direction = Direction::Left;
                        x -= 1;
                    }
                    Direction::Right => {
                        direction = Direction::Top;
                        y += 1;
                    }
                    Direction::Bottom => {
                        direction = Direction::Right;
                        x += 1;
                    }
                    Direction::Left => {
                        direction = Direction::Bottom;
                        y -= 1;
                    }
                }
            }
            1 => {
                match direction {
                    Direction::Top => {
                        direction = Direction::Right;
                        x += 1;
                    }
                    Direction::Right => {
                        direction = Direction::Bottom;
                        y -= 1;
                    }
                    Direction::Bottom => {
                        direction = Direction::Left;
                        x -= 1;
                    }
                    Direction::Left => {
                        direction = Direction::Top;
                        y += 1;
                    }
                }
            }
            _ => unreachable!()
        }
        emergency_hull_painting_robot.inputs.push_back(panels_painted.get(&(x, y)).unwrap_or(&0).clone());
    }
    let mut identifier = vec![vec!['▒'; ((width.0 as f32).abs() as i32 + width.1) as usize + 1]; ((height.0 as f32).abs() as i32 + height.1) as usize + 1];
    for panel in panels_painted {
        if panel.1 == 1 {
            let x: i32 = (panel.0).0 + width.0 * -1;
            let y: i32 = (panel.0).1 + height.0 * -1;
            identifier[y as usize][x as usize] = '█';
        }
    }
    String::from("\n") + &identifier.iter().rev().map(|x| x.iter().join("")).join("\n")
}
