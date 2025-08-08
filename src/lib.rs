//! # xo-core
//!
//! `xo-core` is a fast, reusable, and well-tested Tic-Tac-Toe (Noughts and Crosses) game engine for Rust.
//! It provides all core logic and an unbeatable Minimax AI, with a simple, public API suitable for embedding 
//! in CLI, GUI, or web apps.
//!
//! ## High-Level Summary 
//!
//! - Core types: [`Player`], [`Cell`], [`GameState`], [`MoveError`]
//! - [`GameEngine`] struct to manage game state and moves
//! - Minimax AI: unbeatable computer player with [`GameEngine::get_best_move`]
//!
//! ## Example Usage
//!
//! ```rust
//! use xo_core::{GameEngine, Player, Cell, GameState};
//!
//! let mut game = GameEngine::new();
//! // X plays at 0, O at 4, X at 1, O at 5, X at 2 (X wins)
//! let moves = [0, 4, 1, 5, 2];
//! for m in moves {
//!     game.make_move(m).unwrap();
//! }
//! assert_eq!(game.check_state(), GameState::Win(Player::X));
//! ```
//!
//! ## Board Layout
//!
//! Board cells are indexed as follows:
//!
//! ```text
//!  0 | 1 | 2
//! ---|---|---
//!  3 | 4 | 5
//! ---|---|---
//!  6 | 7 | 8
//! ```
//!
//! ## Integration
//!
//! The engine is detached from I/O and UI, making it easy to use in CLI, GUI, or web applications.
//!
//! ## License
//!
//! MIT

mod types;
mod game_engine;

pub use types::{Player, Cell, GameState, MoveError};
pub use game_engine::GameEngine;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_wins_top_row() {
        let mut game = GameEngine::new();
        game.make_move(0).unwrap(); // X
        game.make_move(3).unwrap(); // O
        game.make_move(1).unwrap(); // X
        game.make_move(4).unwrap(); // O
        game.make_move(2).unwrap(); // X wins
        assert_eq!(game.check_state(), GameState::Win(Player::X));
    }

    #[test]
    fn o_wins_diagonal() {
        let mut game = GameEngine::new();
        game.make_move(0).unwrap(); // X
        game.make_move(2).unwrap(); // O
        game.make_move(1).unwrap(); // X
        game.make_move(4).unwrap(); // O
        game.make_move(3).unwrap(); // X
        game.make_move(6).unwrap(); // O wins diagonally (2,4,6)
        assert_eq!(game.check_state(), GameState::Win(Player::O));
    }

    #[test]
    fn tie_game() {
        let mut game = GameEngine::new();
        let moves = [0,1,2,4,3,5,7,6,8];
        for &i in &moves {
            game.make_move(i).unwrap();
        }
        assert_eq!(game.check_state(), GameState::Tie);
    }

    #[test]
    fn cannot_play_out_of_bounds() {
        let mut game = GameEngine::new();
        assert_eq!(game.make_move(9), Err(MoveError::OutOfBounds));
        assert_eq!(game.make_move(99), Err(MoveError::OutOfBounds));
    }

    #[test]
    fn cannot_play_on_occupied_cell() {
        let mut game = GameEngine::new();
        game.make_move(0).unwrap();
        assert_eq!(game.make_move(0), Err(MoveError::CellOccupied));
    }

    #[test]
    fn board_updates_correctly() {
        let mut game = GameEngine::new();
        game.make_move(0).unwrap();
        let board = game.get_board();
        assert_eq!(board[0], Cell::X);
    }

    #[test]
    fn ai_blocks_win() {
        let mut game = GameEngine::new();
        game.make_move(0).unwrap(); // X
        game.make_move(4).unwrap(); // O
        game.make_move(1).unwrap(); // X
        // O (AI) should block X at 2
        assert_eq!(game.get_best_move(), Some(2));
    }

    #[test]
    fn ai_chooses_winning_move() {
        let mut game = GameEngine::new();
        game.make_move(0).unwrap(); // X
        game.make_move(4).unwrap(); // O
        game.make_move(2).unwrap(); // X
        game.make_move(1).unwrap(); // O
        game.make_move(3).unwrap(); // X
        // O can win by playing at 7
        assert_eq!(game.get_best_move(), Some(7));
    }
}