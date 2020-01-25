use crate::commands::Command;
use crate::game_model::{Board, Robot};

pub fn update_board_from_command(board: &Board, command: &Command) -> Board {
    return match (board.robot, &command) {
        (_, Command::Place { location, facing }) => board.with_robot(Robot {
            location: *location,
            facing: *facing,
        }),
        (None, _) => *board,
        (Some(robot), Command::Move) => {
            board.with_robot(robot.with_position(robot.location.translate(robot.facing)))
        }
        (Some(robot), Command::Rotate(relative_direction)) => {
            board.with_robot(robot.with_facing(robot.facing.rotate(relative_direction)))
        }
        (Some(_robot), Command::Report) => *board,
    };
}

pub fn output_from_command(board: &Board, command: &Command) -> Option<String> {
    return match (board.robot, &command) {
        (Some(robot), Command::Report) => Some(
            format!("{},{},{}", robot.location.x, robot.location.y, robot.facing).to_uppercase(),
        ),
        (_, _) => None,
    };
}

pub fn is_board_valid(board: &Board) -> bool {
    board
        .robot
        .map_or(true, |robot| board.bounds.contains(&robot.location))
}

#[cfg(test)]
mod test {
    use crate::commands::Command;
    use crate::game_model::{Board, Robot};
    use crate::geo::{Square, Vector};
    use crate::geo::Direction::*;
    use crate::geo::RelativeDirection::Left;

    const EMPTY_BOARD: Board = Board {
        bounds: Square {
            bottom_left: Vector { x: 0, y: 0 },
            top_right: Vector { x: 4, y: 4 },
        },
        robot: None,
    };

    mod update_board {
        use crate::commands::Command;
        use crate::game_model::{Board, Robot};
        use crate::geo::Direction::*;
        use crate::geo::RelativeDirection::Left;
        use crate::geo::Vector;

        use super::EMPTY_BOARD;
        use super::super::update_board_from_command;

        #[test]
        fn update_board_move_no_robot() {
            let command = Command::Move;

            let initial_board = EMPTY_BOARD;
            let expected_board = initial_board;

            assert_eq!(
                expected_board,
                update_board_from_command(&initial_board, &command)
            )
        }

        #[test]
        fn update_board_move_with_robot() {
            let command = Command::Move;

            let initial_board = EMPTY_BOARD.with_robot(Robot::new(Vector::new(1, 1), North));
            let expected_board = initial_board.with_robot(Robot::new(Vector::new(1, 2), North));

            assert_eq!(
                expected_board,
                update_board_from_command(&initial_board, &command)
            )
        }

        #[test]
        fn update_board_move_with_robot_on_edge() {
            let command = Command::Move;

            let initial_board = EMPTY_BOARD.with_robot(Robot::new(Vector::new(0, 0), West));
            let expected_board = initial_board.with_robot(Robot::new(Vector::new(-1, 0), West));

            assert_eq!(
                expected_board,
                update_board_from_command(&initial_board, &command)
            )
        }

        #[test]
        fn update_board_rotate_no_robot() {
            let command = Command::Rotate(Left);

            let initial_board = EMPTY_BOARD;
            let expected_board = initial_board;

            assert_eq!(
                expected_board,
                update_board_from_command(&initial_board, &command)
            )
        }

        #[test]
        fn update_board_rotate_with_robot() {
            let command = Command::Rotate(Left);

            let initial_board = EMPTY_BOARD.with_robot(Robot::new(Vector::new(1, 1), North));
            let expected_board = initial_board.with_robot(Robot::new(Vector::new(1, 1), West));

            assert_eq!(
                expected_board,
                update_board_from_command(&initial_board, &command)
            )
        }

        #[test]
        fn update_board_place_no_robot() {
            let command = Command::Place {
                location: Vector::new(1, 1),
                facing: North,
            };

            let initial_board = EMPTY_BOARD;
            let expected_board = initial_board.with_robot(Robot::new(Vector::new(1, 1), North));

            assert_eq!(
                expected_board,
                update_board_from_command(&initial_board, &command)
            )
        }

        #[test]
        fn update_board_place_robot_invalid() {
            let command = Command::Place {
                location: Vector::new(-1, -1),
                facing: North,
            };

            let initial_board = EMPTY_BOARD;
            let expected_board = initial_board.with_robot(Robot::new(Vector::new(-1, -1), North));

            assert_eq!(
                expected_board,
                update_board_from_command(&initial_board, &command)
            )
        }

        #[test]
        fn update_board_place_with_robot() {
            let command = Command::Place {
                location: Vector::new(1, 1),
                facing: North,
            };

            let initial_board = EMPTY_BOARD.with_robot(Robot::new(Vector::new(4, 4), South));
            let expected_board = initial_board.with_robot(Robot::new(Vector::new(1, 1), North));

            assert_eq!(
                expected_board,
                update_board_from_command(&initial_board, &command)
            )
        }
    }

    mod output {
        use crate::commands::Command;
        use crate::game_model::{Board, Robot};
        use crate::geo::Direction::*;
        use crate::geo::RelativeDirection::Left;
        use crate::geo::Vector;

        use super::EMPTY_BOARD;
        use super::super::output_from_command;

        #[test]
        fn output_move_with_robot() {
            let command = Command::Move;

            let board = EMPTY_BOARD.with_robot(Robot::new(Vector::new(1, 1), North));
            let expected_output = None;

            assert_eq!(expected_output, output_from_command(&board, &command))
        }

        #[test]
        fn output_report_with_no_robot() {
            let command = Command::Report;

            let board = EMPTY_BOARD;
            let expected_output = None;

            assert_eq!(expected_output, output_from_command(&board, &command))
        }

        #[test]
        fn output_report_with_robot() {
            let command = Command::Report;

            let board = EMPTY_BOARD.with_robot(Robot::new(Vector::new(1, 1), North));
            let expected_output = Some("1,1,NORTH".to_string());

            assert_eq!(expected_output, output_from_command(&board, &command))
        }
    }

    mod validate {
        use crate::commands::Command;
        use crate::game_model::{Board, Robot};
        use crate::geo::Direction::*;
        use crate::geo::RelativeDirection::Left;
        use crate::geo::Vector;

        use super::EMPTY_BOARD;
        use super::super::is_board_valid;

        #[test]
        fn validate_no_robot() {
            let board = EMPTY_BOARD;
            let expected_valid = true;

            assert_eq!(expected_valid, is_board_valid(&board),)
        }

        #[test]
        fn validate_with_robot() {
            let board = EMPTY_BOARD.with_robot(Robot::new(Vector::new(1, 1), North));
            let expected_valid = true;

            assert_eq!(expected_valid, is_board_valid(&board),)
        }

        #[test]
        fn validate_with_robot_out_of_bounds_north() {
            let board = EMPTY_BOARD.with_robot(Robot::new(Vector::new(1, 5), North));
            let expected_valid = false;

            assert_eq!(expected_valid, is_board_valid(&board),)
        }

        #[test]
        fn validate_with_robot_out_of_bounds_south() {
            let board = EMPTY_BOARD.with_robot(Robot::new(Vector::new(1, -1), North));
            let expected_valid = false;

            assert_eq!(expected_valid, is_board_valid(&board),)
        }

        #[test]
        fn validate_with_robot_out_of_bounds_east() {
            let board = EMPTY_BOARD.with_robot(Robot::new(Vector::new(5, 1), North));
            let expected_valid = false;

            assert_eq!(expected_valid, is_board_valid(&board),)
        }

        #[test]
        fn validate_with_robot_out_of_bounds_west() {
            let board = EMPTY_BOARD.with_robot(Robot::new(Vector::new(-1, 1), North));
            let expected_valid = false;

            assert_eq!(expected_valid, is_board_valid(&board),)
        }
    }
}
