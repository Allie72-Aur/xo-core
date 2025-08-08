use xo_core::{GameEngine, GameState, MoveError, Player};

// --- Main Function to Demonstrate Usage ---
// This main function is provided to show how to use the GameEngine.
// It sets up a simple command-line interface for playing against the AI.
fn main() {
    println!("Welcome to Minimax Tic-Tac-Toe!");
    let mut game = GameEngine::new();

    // The game loop continues until a winner or a tie is determined.
    while !game.is_over() {
        // Print the board for the current turn.
        println!("-----------------");
        print_board(&game);
        println!("-----------------");

        // Player's turn
        if game.current_player == Player::X {
            loop {
                // Get user input.
                let mut input = String::new();
                println!("Player X, enter your move (0-8):");
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Reading Line Should Succeed");

                // Parse the input and attempt to make a move.
                let index: usize = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input! Please enter a number from 0 to 8.");
                        continue;
                    }
                };

                // Handle the result of the `make_move` call.
                match game.make_move(index) {
                    Ok(()) => break,
                    Err(MoveError::OutOfBounds) => {
                        println!("Invalid index! Must be between 0 and 8.");
                    }
                    Err(MoveError::CellOccupied) => {
                        println!("That cell is already taken! Try another one.");
                    }
                }
            }
        } else {
            // CPU's turn
            println!("Player O (CPU) is thinking...");

            // Get the best move from the engine. The `unwrap()` is safe here because
            // the loop condition `!game.is_over()` guarantees there's a move to be made.
            let best_move = game.get_best_move().unwrap();

            // Make the CPU's move.
            game.make_move(best_move).unwrap();
        }
    }

    // After the game loop ends, print the final board and the result.
    println!("--- Final Board ---");
    print_board(&game);
    println!("--- Game Over! ---");

    match game.check_state() {
        GameState::Win(Player::X) => println!("Player X wins!"),
        GameState::Win(Player::O) => println!("Player O (CPU) wins!"),
        GameState::Tie => println!("It's a tie!"),
        _ => unreachable!(),
    }
}

// A helper function to print the game board in a readable 3x3 format.
fn print_board(game: &GameEngine) {
    let board = game.get_board();
    for i in 0..3 {
        println!(
            " {} | {} | {} ",
            board[i * 3],
            board[i * 3 + 1],
            board[i * 3 + 2]
        );
        if i < 2 {
            println!("---|---|---");
        }
    }
}
