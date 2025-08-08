# xo-core

A fast, reusable, and well-documented game engine for Tic-Tac-Toe (Noughts and Crosses) in Rust, featuring a robust Minimax AI.
This library is designed to be embedded in CLI, GUI, and web applications, or used as an educational resource for turn-based game
development and AI.

---

## Features

- **Pure Rust:** No unsafe code, zero dependencies.
- **Robust API:** Exposes core game logic and AI.
- **Minimax AI:** Unbeatable, optimized computer opponent.
- **Easy to Integrate:** Use in any Rust project as a dependency.
- **Well-Documented:** All public items have thorough documentation.
- **Tested:** Includes comprehensive unit tests.

---

## Installation

- Add to your `Cargo.toml`:

```toml
[dependencies]
xo-core = "0.1"
```

*(Replace the version with the latest on [crates.io](https://crates.io/crates/xo-core))*

- Or simply use Cargo:

```bash
cargo add xo-core
```

---

## Getting Started

### Basic Example

```rust
use xo_core::{GameEngine, Player, Cell, GameState};

fn main() {
    let mut game = GameEngine::new();

    // Play moves: X at 0, O at 4, X at 1, O at 5, X at 2 (X wins)
    let moves = [0, 4, 1, 5, 2];
    for m in moves {
        game.make_move(m).unwrap();
    }

    match game.check_state() {
        GameState::Win(Player::X) => println!("X wins!"),
        GameState::Win(Player::O) => println!("O wins!"),
        GameState::Tie => println!("It's a tie!"),
        GameState::InProgress => println!("Game is still in progress."),
    }
}
```

---

### Playing Against the AI

```rust
use xo_core::{GameEngine, Player, Cell};

fn main() {
    let mut game = GameEngine::new();

    // Human is X, AI is O
    loop {
        // Human move (for example, always picking the first empty)
        let board = game.get_board();
        let my_move = board.iter().position(|&c| c == Cell::Empty).unwrap();
        game.make_move(my_move).unwrap();

        // Check for game over
        if game.is_over() {
            break;
        }

        // AI move
        let ai_move = game.get_best_move().unwrap();
        game.make_move(ai_move).unwrap();

        // Check for game over
        if game.is_over() {
            break;
        }
    }

    println!("Final board: {:?}", game.get_board());
}
```

---

## API Overview

### Core Types

- `GameEngine`: The main engine struct. Manages board state, moves, and AI.
- `Player`: Enum for `X` and `O`.
- `Cell`: Enum for `X`, `O`, or `Empty` cell.
- `GameState`: Enum for `Win(Player)`, `Tie`, or `InProgress`.
- `MoveError`: Enum for move errors (`OutOfBounds`, `CellOccupied`).

### Key Methods

- `GameEngine::new()`: Create a new game.
- `make_move(index)`: Attempt a move at given cell (0-8).
- `get_board()`: Get the current board state as `[Cell; 9]`.
- `check_state()`: Check if the game is over, won, tied, or still in progress.
- `is_over()`: Boolean, true if game finished.
- `get_best_move()`: Returns the best move for the current player (Minimax AI).

---

## Board Layout

Cells are indexed left-to-right, top-to-bottom:

```text
 0 | 1 | 2
---|---|---
 3 | 4 | 5
---|---|---
 6 | 7 | 8
```

---

## Error Handling

- Invalid moves return a `Result<(), MoveError>`.
- Errors include:
  - `MoveError::OutOfBounds` — index not in 0..=8
  - `MoveError::CellOccupied` — cell already filled

---

## Example: Integrating With a GUI

You can easily use `xo-core` with web frameworks (e.g., Yew), desktop GUI (e.g., egui, gtk-rs), or in server-side logic.
The engine is detached from any I/O or UI, making it flexible for integration.

---

## Minimum Supported Rust Version

- Rust 1.88 (2024 Edition)

---

## License

MIT

---

## Contribution

Contributions, bug reports, and feature requests welcome!  
Please open an issue or submit a pull request on [GitHub](https://github.com/Allie72-Aur/xo-core).

---

## Acknowledgements

- This project started as a fork of [tic-tac-toe-rs](https://github.com/rogue-87/tic-tac-toe-rs)
  by [rogue-87](https://github.com/rogue-87), whose original implementation and ideas laid the
  foundation for this crate.

---

## See Also

While working on this, I stumbled across a couple of cool projects.
Check them out while you're at it:

- [bitboard_xo](https://github.com/thanadolps/bitboard_xo): XO game implemented in Rust with minimum memory usage
- [tic-tac-toe-rs](https://github.com/rogue-87/tic-tac-toe-rs): A straightforward implementation of Tic-Tac-Toe in Rust and the original inspiration for this crate
