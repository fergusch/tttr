extern crate rand;
extern crate termion;

use std::io;
use rand::Rng;
use board::Board;
use termion::{color, cursor, style, clear};

pub trait Player {

	/// Chooses a space on the board according to
	/// the player's specified method.
	fn choose_space(&self, b: &Board) -> usize;

}

/// Human player that uses terminal
/// input for space selection.
pub struct Human;

impl Human {

	/// Returns a new `Human` object.
	pub fn new() -> Human {
		return Human {};
	}
}

impl Player for Human {

	fn choose_space(&self, b: &Board) -> usize {
		
		// Print input prompt
		println!("\n{}Enter space (1-9){}", style::Bold, style::Reset);
		print!("{}> ", if b.active_player == 1 {'X'} else {'O'});

		// Try to get user input and loop until something valid is entered
		let mut input;
		loop {

			input = String::new();

			// Flush output so input prompt will appear correctly
			io::Write::flush(&mut io::stdout())
				.expect("flush failed!");

			// Get user input
			io::stdin().read_line(&mut input)
				.expect("Failed to read line");

			// Check if the input is non-numeric before parsing
			// If so, prompt the user again
			if !input.trim().parse::<usize>().is_ok() {
				println!("{}{}Enter space (1-9) {}Invalid input!{}", 
					cursor::Up(2), style::Bold,
					color::Fg(color::Red), style::Reset);
				print!("{}>{} ", if b.active_player == 1 {'X'} else {'O'},
					clear::AfterCursor);
				continue;
			}

			// Convert input to integer
			let input: usize = input.trim().parse()
				.expect("Failed to parse");

			// Check if the input is out of range
			// If so, prompt the user again
			if input < 1 || input > 9 {
				println!("{}{}Enter space (1-9) {}That's not a space!{}", 
					cursor::Up(2), style::Bold,
					color::Fg(color::Red), style::Reset);
				print!("{}>{} ", if b.active_player == 1 {'X'} else {'O'},
					clear::AfterCursor);
				continue;
			}

			// Check if the space entered is taken
			// If so, prompt the user again
			if b.space_taken(input) {
				println!("{}{}Enter space (1-9) {}That space is taken!{}", 
					cursor::Up(2), style::Bold,
					color::Fg(color::Red), style::Reset);
				print!("{}>{} ", if b.active_player == 1 {'X'} else {'O'},
					clear::AfterCursor);
				continue;
			}

			// If all checks were passed,
			// return the input and exit the loop
			return input;

		}

	}
}

/// AI player that chooses a random 
/// available space.
pub struct RandomAI;

impl RandomAI {

	/// Returns a new `RandomAI` object.
	pub fn new() -> RandomAI {
		return RandomAI {};
	}

}

impl Player for RandomAI {

	/// Chooses a random space on the board that isn't occupied.
	fn choose_space(&self, b: &Board) -> usize {
		return *rand::thread_rng().choose(&b.open_spaces()).unwrap();
	}

}

/// AI player that chooses a spot using the Minimax algorithm.
pub struct MinimaxAI {
	depth: usize,
}

impl MinimaxAI {

	/// Returns a new `MinimaxAI` with the
	/// given search depth set.
	pub fn new(depth: usize) -> MinimaxAI {
		return MinimaxAI {depth};
	}

	fn minimax(&self, node: (usize, Board), depth: usize, is_max: bool) -> (usize, isize) {
		
		if depth == 0 || node.1.is_terminal().0 > -1 {
			return (node.0, node.1.heuristic());
		}

		let children: Vec<(usize, Board)> = node.1.get_children();

		if is_max {

			let mut value: isize = std::isize::MIN;
			let mut move_: usize = 0;

			for child in children.iter() {
				let mm: (usize, isize) = self.minimax(child.clone(), depth-1, false);
				if mm.1 > value {
					value = mm.1;
					move_ = mm.0;
				}
			}

			return (move_, value);

		} else {

			let mut value: isize = std::isize::MAX;
			let mut move_: usize = 0;
			for child in children.iter() {
				let mm: (usize, isize) = self.minimax(child.clone(), depth-1, true);
				if mm.1 < value {
					value = mm.1;
					move_ = mm.0;
				}
			}
			return (move_, value);

		}

	}

}

impl Player for MinimaxAI {
	fn choose_space(&self, b: &Board) -> usize {
		return self.minimax((0, b.clone()), self.depth, true).0;
	}
}