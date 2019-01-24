extern crate termion;

use board::Board;
use player::Player;
use termion::{cursor, clear, style};

pub struct Game;

fn clear_screen() {
	print!("{}{}", cursor::Up(11), clear::AfterCursor);
}

impl Game {

	pub fn new<A, B>(player1: A, player2: B) where A: Player, B: Player {
		
		// Create a new board
		let mut b: Board = Board::new();

		println!(); // for padding

		// Loop until game over
		loop {

			// Print title and board
			println!("~{}Tic-Tac-Toe{}~\n", style::Bold, style::Reset);
			b.print();

			// Let player 1 choose a space
			b.active_player = 1;
			let input: usize = player1.choose_space(&b);
			b.set_space(input);

			// Refresh the screen after the first input in case this is 2p mode
			clear_screen();
			println!("\n~{}Tic-Tac-Toe{}~\n", style::Bold, style::Reset);
			b.print();

			if b.is_terminal().0 == -1 {

				// Let player 2 choose a space
				b.active_player = 2;
				let input: usize = player2.choose_space(&b);
				b.set_space(input);
			
			}

			clear_screen();

			// check the current state of the board
			let state: (i8, [usize; 3]) = b.is_terminal();

			// if the state is greater than -1, the game is over
			if state.0 > -1 {

				if state.0 > 0 {

					// Re-print the board with the winning spaces highlighted
					// and a winning message
					println!("~{}Tic-Tac-Toe{}~\n", style::Bold, style::Reset);
					b.print_win(state.1);
					println!("\nPlayer {} wins!\n", state.0);

					// Exit the main loop (and program)
					break;

				} else {

					// Re-print the board with "Draw." message
					println!("\n~{}Tic-Tac-Toe{}~\n", style::Bold, style::Reset);
					b.print();
					println!("\nDraw.\n");

					// Exit
					break;

				}

			}

		} 

	}

}