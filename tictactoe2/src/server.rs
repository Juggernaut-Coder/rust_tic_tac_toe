#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]

mod game_utils;
mod constants;
use constants::{BOARD_LEN, SERVER_ADDR, MAX_CLIENT, MAX_MOVES, MOVE_KEYS, GameMessage};
use game_utils::{is_game_won, clear_screen};
use tokio::net::{TcpListener, TcpStream};
use core::net::SocketAddr;
use tokio::task;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use rand::Rng;
use rand::rngs::ThreadRng;

struct Player{
    strm : TcpStream,
    sckaddr : SocketAddr
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    clear_screen();
    let listener = TcpListener::bind(SERVER_ADDR).await?;
    println!("Server listening on port 7878");
    println!("Waiting for players to join");
    let mut players: [Option<Player>; MAX_CLIENT] = [None, None];
    for i in 0..MAX_CLIENT {
        let accepted = listener.accept().await?;
        players[i] = Some(Player { strm: accepted.0, sckaddr: accepted.1});
        println!("Player{} has joined!", i + 1);
    }
    let mut rng: ThreadRng  = rand::thread_rng();
    let mut curr_player_id: usize = rng.gen_range(0..2);
    let mut other_player_id = (curr_player_id + 1) % MAX_CLIENT;
    let mut player = players[curr_player_id].take().unwrap();
    let mut other_player = players[other_player_id].take().unwrap();
    println!("Player{} moves 1st!", curr_player_id + 1);
    let mut board: [[char; BOARD_LEN]; BOARD_LEN] = [[' '; BOARD_LEN]; BOARD_LEN];
    let mut m_cnt: usize = 0;
    let mut move_buf = [0u8; 2];
    while m_cnt < MAX_MOVES {
        let task1 = async {
            other_player.strm.write_u8(GameMessage::Wait.to_u8()).await?;
            Ok::<(), tokio::io::Error>(())
        };
        let task2 = async {
            player.strm.write_u8(GameMessage::YourTurn.to_u8()).await?;
            loop {
                player.strm.read_exact(&mut move_buf).await?;
                player.strm.flush().await?;
                let row: usize = (move_buf[0] as char).to_digit(10).unwrap() as usize;
                let col: usize = (move_buf[1] as char).to_digit(10).unwrap() as usize;
                println!("Player{}'s move is {}{}", curr_player_id + 1, row, col);
                if row < BOARD_LEN && col < BOARD_LEN && board[row][col] == ' ' {
                    println!("Player move is valid");
                    board[row][col] = MOVE_KEYS[curr_player_id];
                    player.strm.write_u8(GameMessage::MoveWithinBounds.to_u8()).await?;
                    break;
                } else {
                    println!("Player move is invalid");
                    player.strm.write_u8(GameMessage::InvalidMoveTryAgain.to_u8()).await?;
                }
            }
            Ok::<(), tokio::io::Error>(())
        };
        let (t1res, t2res) = tokio::join!(task1, task2);
        t1res?;
        t2res?;
        other_player.strm.write_u8(GameMessage::YourTurn.to_u8()).await?;
        println!("Sending board to both players");
        let board_str : String = board.iter().flat_map(|row| row.iter()).collect();
        let task3 = async {
            player.strm.write_all(board_str.as_bytes()).await?;
            Ok::<(), tokio::io::Error>(())
        };
        let task4 = async {
            other_player.strm.write_all(board_str.as_bytes()).await?;
            Ok::<(), tokio::io::Error>(())
        };
        let (t3res, t4res) = tokio::join!(task3, task4);
        t3res?;
        t4res?;
        println!("Checking game status");
        let game_status: Option<char> = is_game_won(&board);
        if game_status == Some('X') {
            send_end_game_msg(&mut player, &mut other_player, GameMessage::Player1HasWon).await?;
            close_stream(&mut player, &mut other_player).await?;
            return Ok(());
        } else if game_status == Some('O') {
            send_end_game_msg(&mut player, &mut other_player, GameMessage::Player2HasWon).await?;
            close_stream(&mut player, &mut other_player).await?;
            return Ok(());
        } else {
            send_end_game_msg(&mut player, &mut other_player, GameMessage::Continue).await?;
        }
        println!("Moving to next turn");
        let temp_player_id = curr_player_id;
        curr_player_id = other_player_id;
        other_player_id = temp_player_id;
        let temp: Player = player;
        player = other_player;
        other_player = temp;
        m_cnt += 1; 
    }
    return Ok(());
}

async fn send_end_game_msg(player1 : &mut Player, player2: &mut Player, msg : GameMessage) -> tokio::io::Result<()> {
    let task1 = async {
        player1.strm.write_u8(msg.to_u8()).await?;
        Ok::<(), tokio::io::Error>(())
    };
    let task2 = async {
        player2.strm.write_u8(msg.to_u8()).await?;
        Ok::<(), tokio::io::Error>(())
    };
    let (t1res, t2res) = tokio::join!(task1, task2);
    t1res?;
    t2res?;
    return Ok(());
}

async fn close_stream(player1 : &mut Player, player2: &mut Player) -> tokio::io::Result<()>  {
    let task1 = async {
        player1.strm.shutdown().await?;
        Ok::<(), tokio::io::Error>(())
    };
    let task2 = async {
        player2.strm.shutdown().await?;
        Ok::<(), tokio::io::Error>(())
    };
    let (t1res, t2res) = tokio::join!(task1, task2);
    t1res?;
    t2res?;
    return Ok(());
}