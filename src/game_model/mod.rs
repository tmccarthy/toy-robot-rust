use crate::geo::{Direction, Square, Vector};
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Robot {
    pub location: Vector,
    pub facing: Direction,
}

impl Robot {
    pub fn new(position: Vector, facing: Direction) -> Robot {
        Robot {
            location: position,
            facing,
        }
    }

    pub fn with_position(self: Robot, position: Vector) -> Robot {
        Robot {
            location: position,
            ..self
        }
    }

    pub fn with_facing(self: Robot, facing: Direction) -> Robot {
        Robot { facing, ..self }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Board {
    pub bounds: Square,
    pub robot: Option<Robot>,
    pub obstacle_locations: HashSet<Vector>,
}

impl Board {
    pub fn empty_with_corner(corner: &Vector) -> Board {
        Board {
            bounds: Square::with_corners(&Vector::new(0, 0), corner),
            robot: None,
            obstacle_locations: HashSet::new(),
        }
    }

    pub fn with_robot(self: &Board, robot: Robot) -> Board {
        Board {
            bounds: self.bounds,
            robot: Some(robot),
            obstacle_locations: self.obstacle_locations.clone(),
        }
    }

    pub fn with_obstacle_at(self: &Board, obstacle_location: Vector) -> Board {
        let mut new_obstacle_locations = self.obstacle_locations.clone();

        new_obstacle_locations.insert(obstacle_location);

        Board {
            obstacle_locations: new_obstacle_locations,
            ..*self
        }
    }
}
