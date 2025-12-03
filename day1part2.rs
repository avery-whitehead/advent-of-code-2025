use eyre::Result;
use std::fs;

enum Direction {
    Left,
    Right,
}

struct Position {
    zeros: isize,
    position: isize,
}

fn main() -> Result<()> {
    let zeros = fs::read_to_string("./src/input")?.lines().fold(
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

fn calc_new_position(position: Position, direction: Direction, mut distance: isize) -> Position {
    if matches!(direction, Direction::Left) {
        distance = -distance;
    }

    let mut new_zeros = position.zeros;
    if position.position == 0 && distance < 0 {
        new_zeros -= 1;
    }

    let mut new_position = position.position + distance;
    new_zeros += new_position.div_euclid(100).abs();
    new_position = new_position.rem_euclid(100);

    if new_position == 0 && distance < 0 {
        new_zeros += 1;
    }

    return Position {
        zeros: new_zeros,
        position: new_position,
    };
}
