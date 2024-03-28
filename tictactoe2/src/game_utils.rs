use text_tables::render;
use crate::constants::BOARD_LEN;
use std::io::{stdout, Write};

pub fn is_game_won(board: &[[char; BOARD_LEN]; BOARD_LEN]) -> Option<char> {
    for i in 0..BOARD_LEN {
        if board[i][0] != ' ' && board[i][0] == board[i][1] && board[i][1] == board[i][2] {
            return Some(board[i][0]);
        }
        if board[0][i] != ' ' && board[0][i] == board[1][i] && board[1][i] == board[2][i] {
            return Some(board[0][i]);
        } 
    }
    if board[0][0] != ' ' && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return Some(board[0][0]);
    }
    if board[0][2] != ' ' && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return Some(board[0][2]);
    }
    return None;
}

pub fn display_board(board: &[[char; BOARD_LEN]; BOARD_LEN]) {
    let board_slices: Vec<&[char]> = board.iter().map(|row| &row[..]).collect();
    let mut out: Vec<u8> = Vec::new();
    render(&mut out, &board_slices).unwrap();
    let board_str: String = String::from_utf8(out).unwrap();
    println!("{}", board_str);
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}
    