use crate::commands::Command::{*};
use crate::commands::parsing::ParsingError::{BadPlaceParameters, UnrecognisedCommand};
use crate::geo::{Direction, Vector};
use crate::geo::RelativeDirection::{*};

use super::Command;

#[derive(Debug, Eq, PartialEq)]
pub enum ParsingError<'a> {
    UnrecognisedCommand(&'a str),
    BadPlaceParameters(&'a str),
}

pub fn parse(input: &str) -> Result<Command, ParsingError> {
    let lowercase_input = input.to_lowercase();

    match lowercase_input.as_ref() {
        "move" => return Ok(Move),
        "left" => return Ok(Rotate(Left)),
        "right" => return Ok(Rotate(Right)),
        _ => {},
    }

    const PLACE_PREFIX: &str = "place ";

    if (lowercase_input.starts_with(PLACE_PREFIX)) {
        parse_place_command(input[(PLACE_PREFIX.len())..].as_ref())
    } else {
        Err(UnrecognisedCommand(input))
    }
}

fn parse_place_command(parameters: &str) -> Result<Command, ParsingError> {
    let split: Vec<&str> = parameters.split(',').collect();

    match split.as_slice() {
        &[raw_x, raw_y, raw_direction] => {
            let maybe_location: Option<Vector> = raw_x.parse::<i16>().and_then(|x| {
                raw_y.parse::<i16>().map(|y| {
                    Vector { x, y }
                })
            }).ok();

            let maybe_facing: Option<Direction> = parse_direction(raw_direction);

            maybe_location.and_then(|location| {
                maybe_facing.map(|facing| {
                    Command::Place { location, facing }
                })
            }).ok_or(BadPlaceParameters(parameters))
        },
        _ => Err(BadPlaceParameters(parameters)),
    }
}

fn parse_direction(raw_direction: &str) -> Option<Direction> {
    match raw_direction.to_lowercase().as_ref() {
        "north" => Some(Direction::North),
        "south" => Some(Direction::South),
        "east" => Some(Direction::East),
        "west" => Some(Direction::West),
        _ => None
    }
}

#[cfg(test)]
mod test {
    use crate::commands::Command::{*};
    use crate::commands::parsing::parse;
    use crate::commands::parsing::ParsingError::{*};
    use crate::geo::RelativeDirection::{*};
    use crate::geo::Vector;
    use crate::geo::Direction::{*};

    #[test]
    fn parse_move() {
        assert_eq!(parse("Move"), Ok(Move))
    }

    #[test]
    fn parse_left() {
        assert_eq!(parse("Left"), Ok(Rotate(Left)))
    }

    #[test]
    fn parse_right() {
        assert_eq!(parse("Right"), Ok(Rotate(Right)))
    }

    #[test]
    fn parse_place_wrong_num_args() {
        assert_eq!(parse("Place 1,ASDF"), Err(BadPlaceParameters("1,ASDF")))
    }

    #[test]
    fn parse_place_bad_direction() {
        assert_eq!(parse("Place 1,1,ASDF"), Err(BadPlaceParameters("1,1,ASDF")))
    }

    #[test]
    fn parse_place_facing_north() {
        assert_eq!(parse("Place 1,1,North"), Ok(Place { location: Vector { x: 1, y: 1 }, facing: North }))
    }

    #[test]
    fn parse_place_facing_south() {
        assert_eq!(parse("Place 1,1,South"), Ok(Place { location: Vector { x: 1, y: 1 }, facing: South }))
    }

    #[test]
    fn parse_place_facing_east() {
        assert_eq!(parse("Place 1,1,East"), Ok(Place { location: Vector { x: 1, y: 1 }, facing: East }))
    }

    #[test]
    fn parse_place_facing_west() {
        assert_eq!(parse("Place 1,1,West"), Ok(Place { location: Vector { x: 1, y: 1 }, facing: West }))
    }

    #[test]
    fn parse_unrecognised() {
        assert_eq!(parse("asdf"), Err(UnrecognisedCommand("asdf")))
    }
}