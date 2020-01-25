#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Eq, PartialEq)]
pub enum RelativeDirection {
    Left,
    Right,
}

use RelativeDirection::{*};
use Direction::{*};
use std::fmt::{Formatter, Error};

impl Direction {
    pub fn rotate(&self, relative_direction: &RelativeDirection) -> Direction {
        match (self, relative_direction) {
            (North, Left) => West,
            (West, Left) => South,
            (South, Left) => East,
            (East, Left) => North,
            (North, Right) => East,
            (East, Right) => South,
            (South, Right) => West,
            (West, Right) => North,
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", match self {
            North => "North",
            South => "South",
            East => "East",
            West => "West",
        })
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Vector {
    pub x: i16,
    pub y: i16,
}

impl Vector {

    pub fn new(x: i16, y: i16) -> Vector {
        Vector { x, y }
    }

    pub fn translate(&self, direction: Direction) -> Vector {
        match direction {
            North => Vector { y: self.y + 1, ..(*self) },
            South => Vector { y: self.y - 1, ..(*self) },
            East => Vector { x: self.x + 1, ..(*self) },
            West => Vector { x: self.x - 1, ..(*self) },
        }
    }

}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Square {
    pub bottom_left: Vector,
    pub top_right: Vector,
}

impl Square {

    pub fn with_corners(corner1: &Vector, corner2: &Vector) -> Square {
        use std::cmp::{min, max};

        let x_min = min(corner1.x, corner2.x);
        let x_max = max(corner1.x, corner2.x);
        let y_min = min(corner1.y, corner2.y);
        let y_max = max(corner1.y, corner2.y);

        Square {
            bottom_left: Vector::new(x_min, y_min),
            top_right: Vector::new(x_max, y_max),
        }
    }

    pub fn contains(self: Square, vector: &Vector) -> bool {
        vector.x >= self.bottom_left.x && vector.x <= self.top_right.x && vector.y >= self.bottom_left.y && vector.y <= self.top_right.y
    }

}

#[cfg(test)]
mod tests {

    mod rotate {
        use crate::geo::Direction::{*};
        use crate::geo::RelativeDirection::{*};

        #[test]
        fn rotate_north_left_gives_west() {
            assert_eq!(North.rotate(&Left), West)
        }

        #[test]
        fn rotate_west_left_gives_south() {
            assert_eq!(West.rotate(&Left), South)
        }

        #[test]
        fn rotate_south_left_gives_east() {
            assert_eq!(South.rotate(&Left), East)
        }

        #[test]
        fn rotate_east_left_gives_north() {
            assert_eq!(East.rotate(&Left), North)
        }

        #[test]
        fn rotate_north_right_gives_east() {
            assert_eq!(North.rotate(&Right), East)
        }

        #[test]
        fn rotate_east_right_gives_south() {
            assert_eq!(East.rotate(&Right), South)
        }

        #[test]
        fn rotate_south_right_gives_west() {
            assert_eq!(South.rotate(&Right), West)
        }

        #[test]
        fn rotate_west_right_gives_north() {
            assert_eq!(West.rotate(&Right), North)
        }

    }

    mod translate {

        use crate::geo::Direction::{*};
        use crate::geo::Vector;

        #[test]
        fn translate_north() {
            assert_eq!(Vector::new(2, 2).translate(North), Vector::new(2, 3))
        }

        #[test]
        fn translate_east() {
            assert_eq!(Vector::new(2, 2).translate(East), Vector::new(3, 2))
        }

        #[test]
        fn translate_south() {
            assert_eq!(Vector::new(2, 2).translate(South), Vector::new(2, 1))
        }

        #[test]
        fn translate_west() {
            assert_eq!(Vector::new(2, 2).translate(West), Vector::new(1, 2))
        }

    }

    mod direction_display {
        use crate::geo::Direction::{*};

        #[test]
        fn display_north() {
            assert_eq!("North", format!("{}", North))
        }

        #[test]
        fn display_east() {
            assert_eq!("East", format!("{}", East))
        }

        #[test]
        fn display_south() {
            assert_eq!("South", format!("{}", South))
        }

        #[test]
        fn display_west() {
            assert_eq!("West", format!("{}", West))
        }

    }

    mod square_contains {
        use crate::geo::Vector;
        use crate::geo::Square;

        #[test]
        fn contains_within_square() {
            let square = Square::with_corners(&Vector::new(0, 0), &Vector::new(4, 4));
            let vector = Vector::new(1, 1);

            assert_eq!(true, square.contains(&vector));
        }

        #[test]
        fn contains_out_of_bounds_north() {
            let square = Square::with_corners(&Vector::new(0, 0), &Vector::new(4, 4));
            let vector = Vector::new(1, 5);

            assert_eq!(false, square.contains(&vector));
        }

        #[test]
        fn contains_out_of_bounds_south() {
            let square = Square::with_corners(&Vector::new(0, 0), &Vector::new(4, 4));
            let vector = Vector::new(1, -1);

            assert_eq!(false, square.contains(&vector));
        }

        #[test]
        fn contains_out_of_bounds_east() {
            let square = Square::with_corners(&Vector::new(0, 0), &Vector::new(4, 4));
            let vector = Vector::new(5, 1);

            assert_eq!(false, square.contains(&vector));
        }

        #[test]
        fn contains_out_of_bounds_west() {
            let square = Square::with_corners(&Vector::new(0, 0), &Vector::new(4, 4));
            let vector = Vector::new(-1, 1);

            assert_eq!(false, square.contains(&vector));
        }

    }

}