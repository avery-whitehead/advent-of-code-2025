use std::{error::Error, fs};

enum Direction {
    Left,
    Right,
}

struct Position {
    zeros: usize,
    position: isize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let zeros = fs::read_to_string("./inputs/day1")?.lines().fold(
        Some(Position {
            zeros: 0,
            position: 50,
        }),
        |position, rotation| {
            Some(calc_new_position(
                position?,
                get_direction(rotation)?,
                get_distance(rotation)?,
            ))
        },
    );

    if let Some(result) = zeros {
        println!("{}", result.zeros);
    }

    Ok(())
}

fn get_direction(rotation: &str) -> Option<Direction> {
    match rotation.chars().nth(0) {
        Some('L') => Some(Direction::Left),
        Some('R') => Some(Direction::Right),
        _ => None,
    }
}

fn get_distance(rotation: &str) -> Option<isize> {
    rotation[1..].parse::<isize>().ok()
}

fn calc_new_position(position: Position, direction: Direction, distance: isize) -> Position {
    match direction {
        Direction::Left => {
            let new_position = (position.position - distance) % 100;
            if new_position == 0 {
                Position {
                    zeros: position.zeros + 1,
                    position: new_position,
                }
            } else {
                Position {
                    zeros: position.zeros,
                    position: new_position,
                }
            }
        }
        Direction::Right => {
            let new_position = (position.position + distance) % 100;
            if new_position == 0 {
                Position {
                    zeros: position.zeros + 1,
                    position: new_position,
                }
            } else {
                Position {
                    zeros: position.zeros,
                    position: new_position,
                }
            }
        }
    }
}
