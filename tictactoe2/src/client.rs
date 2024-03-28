#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]

mod constants;
mod game_utils;
use game_utils::{display_board, clear_screen};
use constants::{BOARD_LEN, SERVER_ADDR, MAX_MOVES, GameMessage};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io::{stdin, stdout};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    clear_screen();
    let mut strm: TcpStream = TcpStream::connect(SERVER_ADDR).await?;
    println!("Connected to the game server");
    let mut board: [[char; BOARD_LEN]; BOARD_LEN] = [[' '; BOARD_LEN]; BOARD_LEN];
    display_board(&board);
    let mut m_cnt: usize = 0;
    while m_cnt <  MAX_MOVES {
        let your_move = strm.read_u8().await?;
        if your_move == GameMessage::YourTurn.to_u8() {
            println!("Enter your move (row and column, e.g., '12'): ");
            loop {
                let mut move_coord = String::new();
                stdin().read_line(&mut move_coord)?;
                strm.write_all(&move_coord.trim().as_bytes()[0..2]).await?;
                let move_success_u8 = strm.read_u8().await?;
                if move_success_u8 == GameMessage::InvalidMoveTryAgain.to_u8() {
                    println!("{}", GameMessage::InvalidMoveTryAgain.as_str());
                } else {
                    println!("here");
                    break;
                }
            }
        } else {
            println!("{}", GameMessage::Wait.as_str());
            let end_of_turn_msg = strm.read_u8().await?;
        }
        receive_board(&mut strm, &mut board).await?;
        clear_screen();
        display_board(&board);
        let game_msg = strm.read_u8().await?;
        if game_msg == GameMessage::Player1HasWon.to_u8() {
            println!("{}", GameMessage::Player1HasWon.as_str());
            strm.shutdown().await?;
            return Ok(());
        } else if game_msg == GameMessage::Player2HasWon.to_u8() {
            println!("{}", GameMessage::Player2HasWon.as_str());
            strm.shutdown().await?;
            return Ok(());
        }
        m_cnt += 1;
    }
    println!("Game is Drawn!");
    strm.shutdown().await?;
    return Ok(());
}

async fn receive_board(strm: &mut TcpStream, board: &mut [[char; BOARD_LEN]; BOARD_LEN]) -> tokio::io::Result<()> {
    let mut buf: [u8; 9] = [3u8; 9];
    strm.read_exact(&mut buf).await?;
    for (i, &byte) in buf.iter().enumerate() {
        let row = i / BOARD_LEN;
        let col = i % BOARD_LEN;
        board[row][col] = byte as char;
    }
    return Ok(());
}