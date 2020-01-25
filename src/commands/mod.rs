use crate::geo::{Direction, RelativeDirection, Vector};

pub mod parsing;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Place { location: Vector, facing: Direction },
    PlaceObject,
    Move,
    Rotate(RelativeDirection),
    Report,
}
