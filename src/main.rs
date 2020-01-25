mod commands;
mod game_execution;
mod game_model;
mod geo;

use crate::game_model::Board;
use crate::geo::Vector;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    let mut board = Board::empty_with_corner(&Vector::new(4, 4));

    for line in stdin.lock().lines() {
        let raw_user_input = line.unwrap();

        let command_or_error = crate::commands::parsing::parse(raw_user_input.as_str());

        command_or_error
            .map(|command| {
                crate::game_execution::output_from_command(&board, &command).map(|output| {
                    println!("{}", output);
                });

                let new_board = crate::game_execution::update_board_from_command(&board, &command);

                if crate::game_execution::is_board_valid(&new_board) {
                    board = new_board
                };
            })
            .unwrap_or_else(|parsing_error| eprintln!("{}", parsing_error))
    }
}
