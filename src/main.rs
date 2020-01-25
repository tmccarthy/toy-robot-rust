use std::io;
use std::io::prelude::*;

use crate::commands::parsing::parse_command;
use crate::game_execution::{is_board_valid, output_from_command, update_board_from_command};
use crate::game_model::Board;
use crate::geo::Vector;

mod commands;
mod game_execution;
mod game_model;
mod geo;

fn main() {
    let stdin = io::stdin();

    let mut board = Board::empty_with_corner(&Vector::new(4, 4));

    for line in stdin.lock().lines() {
        let raw_user_input = line.unwrap();

        let command_or_error = parse_command(raw_user_input.as_str());

        command_or_error
            .map(|command| {
                output_from_command(&board, &command).map(|output| {
                    println!("{}", output);
                });

                let new_board = update_board_from_command(&board, &command);

                if is_board_valid(&new_board) {
                    board = new_board
                };
            })
            .unwrap_or_else(|parsing_error| eprintln!("{}", parsing_error))
    }
}
