#[derive(Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Eq, PartialEq)]
enum RelativeDirection {
    Left,
    Right,
}

use RelativeDirection::{*};
use Direction::{*};

impl Direction {
    fn rotate(&self, relative_direction: RelativeDirection) -> Direction {
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

#[derive(Debug, Eq, PartialEq)]
struct Vector {
    x: i16,
    y: i16,
}

impl Vector {

    fn translate(&self, direction: Direction) -> Vector {
        match direction {
            North => Vector { y: self.y + 1, ..(*self) },
            South => Vector { y: self.y - 1, ..(*self) },
            East => Vector { x: self.x + 1, ..(*self) },
            West => Vector { x: self.x - 1, ..(*self) },
        }
    }

}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod rotate {
        use crate::geo::Direction::{*};
        use crate::geo::RelativeDirection::{*};

        #[test]
        fn rotate_north_left_gives_west() {
            assert_eq!(North.rotate(Left), West)
        }

        #[test]
        fn rotate_west_left_gives_south() {
            assert_eq!(West.rotate(Left), South)
        }

        #[test]
        fn rotate_south_left_gives_east() {
            assert_eq!(South.rotate(Left), East)
        }

        #[test]
        fn rotate_east_left_gives_north() {
            assert_eq!(East.rotate(Left), North)
        }

        #[test]
        fn rotate_north_right_gives_east() {
            assert_eq!(North.rotate(Right), East)
        }

        #[test]
        fn rotate_east_right_gives_south() {
            assert_eq!(East.rotate(Right), South)
        }

        #[test]
        fn rotate_south_right_gives_west() {
            assert_eq!(South.rotate(Right), West)
        }

        #[test]
        fn rotate_west_right_gives_north() {
            assert_eq!(West.rotate(Right), North)
        }

    }

    mod translate {

        use crate::geo::Direction::{*};
        use crate::geo::Vector;

        #[test]
        fn translate_north() {
            assert_eq!(Vector {x: 2, y: 2}.translate(North), Vector {x: 2, y: 3} )
        }

        #[test]
        fn translate_east() {
            assert_eq!(Vector {x: 2, y: 2}.translate(East), Vector {x: 3, y: 2} )
        }

        #[test]
        fn translate_south() {
            assert_eq!(Vector {x: 2, y: 2}.translate(South), Vector {x: 2, y: 1} )
        }

        #[test]
        fn translate_west() {
            assert_eq!(Vector {x: 2, y: 2}.translate(West), Vector {x: 1, y: 2} )
        }

    }

}