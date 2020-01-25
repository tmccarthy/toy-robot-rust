use crate::geo::{Direction, Vector, Square};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Robot {
    pub location: Vector,
    pub facing: Direction,
}

impl Robot {
    pub fn new(position: Vector, facing: Direction) -> Robot {
        Robot { location: position, facing }
    }

    pub fn with_position(self: Robot, position: Vector) -> Robot {
        Robot { location: position, ..self }
    }

    pub fn with_facing(self: Robot, facing: Direction) -> Robot {
        Robot { facing, ..self }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Board {
    pub bounds: Square,
    pub robot: Option<Robot>,
}

impl Board {
    pub fn empty_with_corner(corner: &Vector) -> Board {
        Board {
            bounds: Square::with_corners(&Vector::new(0, 0), corner),
            robot: None,
        }
    }

    pub fn with_robot(self: Board, robot: Robot) -> Board {
        Board {
            robot: Some(robot),
            ..self
        }
    }
}
