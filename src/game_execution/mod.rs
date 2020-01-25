use crate::game_model::{Board, Robot};
use crate::commands::Command;

pub fn update_board_from_command(board: Board, command: &Command) -> Board {
    return match (board.robot, &command) {
        (_, Command::Place { location, facing }) => board.with_robot(Robot { position: *location, facing: *facing }),
        (None, _) => board,
        (Some(robot), Command::Move) => board.with_robot(robot.with_position(robot.position.translate(robot.facing))),
        (Some(robot), Command::Rotate(relative_direction)) => board.with_robot(robot.with_facing(robot.facing.rotate(relative_direction))),
        (Some(robot), Command::Report) => board,
    }
}

#[cfg(test)]
mod test {

    use crate::game_model::{Board, Robot};
    use crate::geo::Vector;
    use crate::commands::Command;
    use crate::geo::Direction::{*};
    use crate::geo::RelativeDirection::Left;

    const EMPTY_BOARD: Board = Board {
        corner: Vector { x: 4, y: 4 },
        robot: None,
    };

    #[test]
    fn update_board_move_no_robot() {
        let command = Command::Move;

        let initial_board = EMPTY_BOARD;
        let expected_board = initial_board;

        assert_eq!(expected_board, super::update_board_from_command(initial_board, &command))
    }

    #[test]
    fn update_board_move_with_robot() {
        let command = Command::Move;

        let initial_board = EMPTY_BOARD.with_robot(Robot::new(Vector::new(1, 1), North));
        let expected_board = initial_board.with_robot(Robot::new(Vector::new(1, 2), North));

        assert_eq!(expected_board, super::update_board_from_command(initial_board, &command))
    }

    #[test]
    fn update_board_move_with_robot_on_edge() {
        let command = Command::Move;

        let initial_board = EMPTY_BOARD.with_robot(Robot::new(Vector::new(0, 0), West));
        let expected_board = initial_board.with_robot(Robot::new(Vector::new(-1, 0), West));

        assert_eq!(expected_board, super::update_board_from_command(initial_board, &command))
    }

    #[test]
    fn update_board_rotate_no_robot() {
        let command = Command::Rotate(Left);

        let initial_board = EMPTY_BOARD;
        let expected_board = initial_board;

        assert_eq!(expected_board, super::update_board_from_command(initial_board, &command))
    }

    #[test]
    fn update_board_rotate_with_robot() {
        let command = Command::Rotate(Left);

        let initial_board = EMPTY_BOARD.with_robot(Robot::new(Vector::new(1, 1), North));
        let expected_board = initial_board.with_robot(Robot::new(Vector::new(1, 1), West));

        assert_eq!(expected_board, super::update_board_from_command(initial_board, &command))
    }

    #[test]
    fn update_board_place_no_robot() {
        let command = Command::Place {
            location: Vector::new(1, 1),
            facing: North,
        };

        let initial_board = EMPTY_BOARD;
        let expected_board = initial_board.with_robot(Robot::new(Vector::new(1, 1), North));

        assert_eq!(expected_board, super::update_board_from_command(initial_board, &command))
    }

    #[test]
    fn update_board_place_robot_invalid() {
        let command = Command::Place {
            location: Vector::new(-1, -1),
            facing: North,
        };

        let initial_board = EMPTY_BOARD;
        let expected_board = initial_board.with_robot(Robot::new(Vector::new(-1, -1), North));

        assert_eq!(expected_board, super::update_board_from_command(initial_board, &command))
    }

    #[test]
    fn update_board_place_with_robot() {
        let command = Command::Place {
            location: Vector::new(1, 1),
            facing: North,
        };

        let initial_board = EMPTY_BOARD.with_robot(Robot::new(Vector::new(4, 4), South));
        let expected_board = initial_board.with_robot(Robot::new(Vector::new(1, 1), North));

        assert_eq!(expected_board, super::update_board_from_command(initial_board, &command))
    }

}