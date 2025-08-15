use crate::types::{Cell, GameState, MoveError, Player};

/// The core Tic-Tac-Toe game engine.
///
/// This struct manages the board, enforces rules, and provides
/// an unbeatable AI opponent using the Minimax algorithm if enabled.
///
/// # Board Representation
/// The board is stored internally as a flat array of 9 cells.  
/// Indices map to positions like this:
///
/// ```text
///  0 | 1 | 2
/// -----------
///  3 | 4 | 5
/// -----------
///  6 | 7 | 8
/// ```
///
/// # Game Modes
/// - **Human vs Human:** Both players call [`make_move`] manually.
/// - **Human vs AI:** calls [`make_move`], then queries
///   [`get_best_move`] to find the AI’s move and applies it with [`make_move`].
///
/// # Examples
///
/// ## Human vs Human
/// ```
/// use xo_core::{GameEngine, Player};
///
/// let mut game = GameEngine::with_ai(false);
///
/// game.make_move(0).unwrap(); // X plays top-left
/// game.make_move(4).unwrap(); // O plays center
///
/// assert_eq!(game.current_player, Player::X);
/// ```
///
/// ## Human vs AI
/// ```
/// use xo_core::GameEngine;
///
/// let mut game = GameEngine::with_ai(true);
///
/// game.make_move(0).unwrap(); // Human plays top-left
///
/// if let Some(ai_move) = game.get_best_move() {
///     game.make_move(ai_move).unwrap(); // Apply AI move
/// }
/// ```
pub struct GameEngine {
    board: [Cell; 9],
    /// The player whose turn it is.
    pub current_player: Player,
    /// Whether the AI is enabled.
    ///
    /// - `true`: Single-player vs AI
    /// - `false`: Human vs Human
    pub ai_enabled: bool,
}

impl GameEngine {
    /// Creates a new instance of the game engine with an empty board.
    ///
    /// The game always starts with `Player::X`.  
    /// By default, AI is **enabled**.
    ///
    /// # Example
    /// ```
    /// use xo_core::{GameEngine, Player, Cell};
    ///
    /// let game = GameEngine::new();
    /// assert_eq!(game.current_player, Player::X);
    /// assert!(game.get_board().iter().all(|&c| c == Cell::Empty));
    /// assert!(game.ai_enabled);
    /// ```
    pub fn new() -> Self {
        Self {
            board: [Cell::Empty; 9],
            current_player: Player::X,
            ai_enabled: true,
        }
    }

    /// Creates a new instance of the game engine with an option to disable AI.
    ///
    /// # Parameters
    /// - `ai_enabled`:  
    ///   - `true`: Single-player vs AI  
    ///   - `false`: Human vs Human
    ///
    /// # Example
    /// ```
    /// use xo_core::GameEngine;
    ///
    /// let game = GameEngine::with_ai(false);
    /// assert!(!game.ai_enabled);
    /// ```
    pub fn with_ai(ai_enabled: bool) -> Self {
        Self {
            board: [Cell::Empty; 9],
            current_player: Player::X,
            ai_enabled,
        }
    }

    /// Returns a reference to the current board.
    pub fn get_board(&self) -> &[Cell; 9] {
        &self.board
    }

    /// Attempts to make a move for the current player at the given board index.
    ///
    /// # Parameters
    /// - `index`: The 0-based cell index (0–8).
    ///
    /// # Returns
    /// - `Ok(())` if the move was made successfully.
    /// - `Err(MoveError)` if the move is invalid:
    ///   - `MoveError::OutOfBounds` if `index >= 9`
    ///   - `MoveError::CellOccupied` if the cell already has a mark
    ///
    /// # Example
    /// ```
    /// use xo_core::{GameEngine, MoveError};
    ///
    /// let mut game = GameEngine::new();
    /// assert!(game.make_move(0).is_ok());
    /// assert_eq!(game.make_move(0), Err(MoveError::CellOccupied));
    /// ```
    pub fn make_move(&mut self, index: usize) -> Result<(), MoveError> {
        // First, check if the index is within the valid range of the board.
        if index >= 9 {
            return Err(MoveError::OutOfBounds);
        }

        // Then, check if the cell is already occupied.
        if self.board[index] != Cell::Empty {
            return Err(MoveError::CellOccupied);
        }

        // Place the current player's mark on the board.
        match self.current_player {
            Player::X => self.board[index] = Cell::X,
            Player::O => self.board[index] = Cell::O,
        }

        // Switch to the other player for the next turn.
        self.current_player = self.current_player.opponent();
        Ok(())
    }

    /// Returns the current state of the game.
    ///
    /// Possible values:
    /// - `GameState::InProgress`
    /// - `GameState::Tie`
    /// - `GameState::Won(Player::X)`
    /// - `GameState::Won(Player::O)`
    pub fn check_state(&self) -> GameState {
        Self::check_board_state(&self, self.board)
    }

    /// Returns `true` if the game is finished (either win or draw).
    pub fn is_over(&self) -> bool {
        !matches!(self.check_state(), GameState::InProgress)
    }

    /// Calculates the best move for the current player using Minimax with pruning.
    ///
    /// Returns:
    /// - `Some(index)` for the best move when AI is enabled.
    /// - `None` if the game is over or AI is disabled.
    ///
    /// # Example
    /// ```
    /// use xo_core::GameEngine;
    ///
    /// let game = GameEngine::with_ai(false);
    /// assert_eq!(game.get_best_move(), None); // AI disabled
    /// ```
    /// # Example with AI
    /// ```
    /// use xo_core::GameEngine;
    ///
    /// let mut game = GameEngine::new();
    /// game.make_move(0).unwrap(); // X
    /// game.make_move(4).unwrap(); // O
    /// game.make_move(1).unwrap(); // X
    ///
    /// // O should block X's winning move at index 2
    /// assert_eq!(game.get_best_move(), Some(2));
    /// ```
    ///
    pub fn get_best_move(&self) -> Option<usize> {
        // If the game is over, no move can be made.
        // furthermore, if AI is disabled, return None.
        if !self.ai_enabled || self.is_over() {
            return None;
        }

        let mut best_score = -i32::MAX;
        let mut best_move: Option<usize> = None;

        // The current player is the maximizing player for the Minimax algorithm.
        let maximizing_player = self.current_player;

        // Iterate through each cell on the board.
        for i in 0..9 {
            // Only consider empty cells as potential moves.
            if self.board[i] == Cell::Empty {
                // Create a temporary clone of the board to simulate the move.
                let mut temp_board = self.board;
                match maximizing_player {
                    Player::X => temp_board[i] = Cell::X,
                    Player::O => temp_board[i] = Cell::O,
                }

                // Recursively call the minimax function to evaluate the score of this move.
                let score = self.minimax_with_pruning(
                    temp_board,
                    maximizing_player.opponent(),
                    -i32::MAX,
                    i32::MAX,
                );

                // If this move's score is better than the current best score,
                // update the best score and the best move index.
                if score > best_score {
                    best_score = score;
                    best_move = Some(i);
                }
            }
        }
        best_move
    }

    /// The Minimax algorithm with Alpha-Beta pruning, implemented recursively.
    ///
    /// This is a private helper method that evaluates the game tree to find the
    /// best possible move.
    /// - `board`: The current state of the game board.
    /// - `player`: The player whose turn it is to evaluate.
    /// - `alpha`: The best score for the maximizing player.
    /// - `beta`: The best score for the minimizing player.
    ///
    /// Returns an integer score for the current board state.
    fn minimax_with_pruning(
        &self,
        board: [Cell; 9],
        player: Player,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        // Check the state of the board and return a score if the game is over.
        let state = self.check_board_state(board);
        match state {
            GameState::Win(winner) => {
                // Return a positive score for a win, negative for a loss.
                // The score is large to represent a definite win/loss.
                return if winner == self.current_player {
                    10
                } else {
                    -10
                };
            }
            GameState::Tie => return 0,
            GameState::InProgress => {}
        }

        // Find all available moves (empty cells).
        let available_moves: Vec<usize> = board
            .iter()
            .enumerate()
            .filter_map(
                |(i, &cell)| {
                    if cell == Cell::Empty {
                        Some(i)
                    } else {
                        None
                    }
                },
            )
            .collect();

        // If there are no available moves, it's a tie.
        if available_moves.is_empty() {
            return 0;
        }

        // The current player is either the maximizing or minimizing player in this subtree.
        let current_player_is_maximizing = player == self.current_player;

        if current_player_is_maximizing {
            let mut max_eval = -i32::MAX;
            for &move_index in &available_moves {
                // Simulate the move.
                let mut temp_board = board;
                match player {
                    Player::X => temp_board[move_index] = Cell::X,
                    Player::O => temp_board[move_index] = Cell::O,
                }

                // Recursively call minimax for the opponent.
                let eval = self.minimax_with_pruning(temp_board, player.opponent(), alpha, beta);

                // Update the maximum score.
                max_eval = max_eval.max(eval);

                // Update alpha for the maximizing player.
                alpha = alpha.max(eval);

                // Alpha-beta pruning condition.
                if beta <= alpha {
                    break;
                }
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for &move_index in &available_moves {
                // Simulate the move.
                let mut temp_board = board;
                match player {
                    Player::X => temp_board[move_index] = Cell::X,
                    Player::O => temp_board[move_index] = Cell::O,
                }

                // Recursively call minimax for the opponent.
                let eval = self.minimax_with_pruning(temp_board, player.opponent(), alpha, beta);

                // Update the minimum score.
                min_eval = min_eval.min(eval);

                // Update beta for the minimizing player.
                beta = beta.min(eval);

                // Alpha-beta pruning condition.
                if beta <= alpha {
                    break;
                }
            }
            min_eval
        }
    }

    /// A helper function to check the state of a given board.
    /// This is used internally by the Minimax algorithm.
    fn check_board_state(&self, board: [Cell; 9]) -> GameState {
        // Define all possible winning combinations (rows, columns, diagonals).
        let winning_combinations = [
            // Rows
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            // Columns
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            // Diagonals
            [0, 4, 8],
            [2, 4, 6],
        ];

        // Iterate through each winning combination to check for a win.
        for combination in &winning_combinations {
            let cell_1 = board[combination[0]];
            let cell_2 = board[combination[1]];
            let cell_3 = board[combination[2]];

            // If the cells are not empty and all three are the same, we have a winner.
            if cell_1 != Cell::Empty && cell_1 == cell_2 && cell_2 == cell_3 {
                // Determine the winning player based on the cell's state.
                return match cell_1 {
                    Cell::X => GameState::Win(Player::X),
                    Cell::O => GameState::Win(Player::O),
                    _ => unreachable!(),
                };
            }
        }

        // If no winner is found, check if the board is full.
        if !board.iter().any(|&cell| cell == Cell::Empty) {
            return GameState::Tie;
        }

        // If neither a win nor a tie, the game is still ongoing.
        GameState::InProgress
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from this module

    #[test]
    fn x_can_win() {
        let mut game = GameEngine::new();
        game.make_move(0).unwrap(); // X
        game.make_move(3).unwrap(); // O
        game.make_move(1).unwrap(); // X
        game.make_move(4).unwrap(); // O
        game.make_move(2).unwrap(); // X wins
        assert_eq!(game.check_state(), GameState::Win(Player::X));
    }

    #[test]
    fn tie_game() {
        let mut game = GameEngine::new();
        let moves = [0, 1, 2, 4, 3, 5, 7, 6, 8];
        for &i in &moves {
            game.make_move(i).unwrap();
        }
        assert_eq!(game.check_state(), GameState::Tie);
    }

    #[test]
    fn invalid_move_out_of_bounds() {
        let mut game = GameEngine::new();
        assert_eq!(game.make_move(9), Err(MoveError::OutOfBounds));
    }

    #[test]
    fn invalid_move_occupied() {
        let mut game = GameEngine::new();
        game.make_move(0).unwrap();
        assert_eq!(game.make_move(0), Err(MoveError::CellOccupied));
    }

    #[test]
    fn minimax_ai_blocks_win() {
        let mut game = GameEngine::new();
        game.make_move(0).unwrap(); // X
        game.make_move(4).unwrap(); // O
        game.make_move(1).unwrap(); // X
                                    // O (AI) should now block X at 2
        assert_eq!(game.get_best_move(), Some(2));
    }

    #[test]
    fn human_vs_human_mode() {
        let game = GameEngine::with_ai(false);
        assert_eq!(game.get_best_move(), None); // AI disabled
    }
}
