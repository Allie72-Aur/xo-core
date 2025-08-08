// --- Data Structures for the Game Engine ---

/// Represents the two possible players in the game.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

impl Player {
    /// Returns the opponent of the current player.
    pub fn opponent(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

/// Represents the state of a single cell on the board.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    X,
    O,
    Empty,
}

// The standard library's `fmt` module is imported for printing and formatting.
use std::fmt;

impl fmt::Display for Cell {
    /// Implements the `Display` trait to allow a `Cell` to be printed cleanly.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::X => write!(f, "X"),
            Cell::O => write!(f, "O"),
            Cell::Empty => write!(f, "."),
        }
    }
}

/// Represents the overall state of the game.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Win(Player),
    Tie,
    InProgress,
}

/// Errors that can occur when attempting to make a move.
#[derive(Debug, PartialEq)]
pub enum MoveError {
    OutOfBounds,
    CellOccupied,
}
