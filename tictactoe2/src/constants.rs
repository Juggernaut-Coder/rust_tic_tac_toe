pub const BOARD_LEN: usize = 3;
pub const SERVER_ADDR : &str = "127.0.0.1:7878";
pub const MAX_CLIENT : usize = 2;
pub const MAX_MOVES : usize = 9;
pub const MOVE_KEYS : [char; MAX_CLIENT] = ['X', 'O'];

#[derive(Copy, Clone)]
pub enum GameMessage {
    YourTurn = 1,
    Wait = 0,
    MoveWithinBounds = 2,
    InvalidMoveTryAgain = 3,
    Player1HasWon = 4,
    Player2HasWon = 5,
    Continue = 6,
}

impl GameMessage {

    pub fn to_u8(self) -> u8 {
        self as u8
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            GameMessage::YourTurn => "Your turn to move",
            GameMessage::Wait => "Wait! Its opponent's turn",
            GameMessage::MoveWithinBounds => "move within bounds",
            GameMessage::InvalidMoveTryAgain => "Invalid move, try again.",
            GameMessage::Player1HasWon => "Player1 has won!",
            GameMessage::Player2HasWon => "Player2 has won!",
            GameMessage::Continue => "Continue",
        }
    }
}