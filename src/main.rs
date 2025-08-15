use std::io;
use xo_core::{GameEngine, GameState, MoveError, Player};

// --- Main Function to Demonstrate Usage ---
// This main function is provided to show how to use the GameEngine.
// It sets up a simple command-line interface for playing against the AI.
fn main() {
    println!("Welcome! Let's play Tic-Tac-Toe!");
    println!("Choose Game Mode:");
    println!("\t1. Single-Player vs AI");
    println!("\t2. Two-Player Mode");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mode = input.trim().to_string();

    if mode != "1" && mode != "2" {
        println!("Invalid choice! Exiting.");
        return;
    }

    let mut player_choice = "1"; // Default first player
    if mode == "1" {
        println!("Choose your player:");
        println!("\t1. Player X (Goes first)");
        println!("\t2. Player O (Goes second)");

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        player_choice = input.trim();

        if player_choice != "1" && player_choice != "2" {
            println!("Invalid choice! Exiting.");
            return;
        }
    }

    let mut game = GameEngine::with_ai(mode == "1");

    // If player chose O, AI (X) should move first
    if mode == "1" && player_choice == "2" {
        let best_move = game.get_best_move().unwrap();
        game.make_move(best_move).unwrap();
    }

    // Game loop
    while !game.is_over() {
        // Print the board for the current turn.
        println!("-----------------");
        print_board(&game);
        println!("-----------------");

        match mode.as_str() {
            "1" => single_player_turn(&mut game, player_choice),
            "2" => two_player_turn(&mut game),
            _ => unreachable!(),
        }
    }

    // After the game loop ends, print the final board and the result.
    println!("--- Final Board ---");
    print_board(&game);
    println!("--- Game Over! ---");

    match game.check_state() {
        GameState::Win(Player::X) => println!("Player X wins!"),
        GameState::Win(Player::O) => println!("Player O wins!"),
        GameState::Tie => println!("It's a tie!"),
        _ => unreachable!(),
    }
}

fn single_player_turn(game: &mut GameEngine, player_choice: &str) {
    let human_player = if player_choice == "1" {
        Player::X
    } else {
        Player::O
    };

    if game.current_player == human_player {
        loop {
            let mut input = String::new();
            println!("Your turn ({:#?}), enter move 0-8:", human_player);
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let index: usize = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input! Please enter a number from 0 to 8.");
                    continue;
                }
            };

            match game.make_move(index) {
                Ok(()) => break,
                Err(MoveError::OutOfBounds) => println!("Invalid index! Must be 0-8."),
                Err(MoveError::CellOccupied) => println!("Cell already taken! Try another."),
            }
        }
    } else {
        println!("AI is thinking...");
        let best_move = game.get_best_move().unwrap();
        game.make_move(best_move).unwrap();
    }
}

fn two_player_turn(game: &mut GameEngine) {
    loop {
        let mut input = String::new();
        println!("Player {:?}, enter your move (0-8):", game.current_player);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let index: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number from 0 to 8.");
                continue;
            }
        };

        match game.make_move(index) {
            Ok(()) => break,
            Err(MoveError::OutOfBounds) => println!("Invalid index! Must be 0-8."),
            Err(MoveError::CellOccupied) => println!("Cell already taken! Try another."),
        }
    }
}

// Print board helper
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
