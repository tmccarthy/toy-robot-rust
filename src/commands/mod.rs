mod parsing;

use crate::geo::{RelativeDirection, Direction, Vector};

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Place {
        location: Vector,
        facing: Direction
    },
    Move,
    Rotate(RelativeDirection),
    Report,
}
