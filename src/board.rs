use termion::{color, style};

/// Struct for a `Board`.
/// - **board:** 3x3 matrix representing the board
/// - **active_player:** the currently active player
pub struct Board {
	board: [[u8; 3]; 3],
	pub active_player: u8
}

impl Clone for Board {
	fn clone(&self) -> Board {
		return Board {board: self.board.clone(), active_player: self.active_player};
	}
}

impl Board {

	/// Creates a new empty `Board`.
	pub fn new() -> Board {
		return Board {board: [[0,0,0], [0,0,0], [0,0,0]], active_player: 0}
	}

	/// Returns an array of the spaces in the given row.
	pub fn get_row_nums(i: usize) -> [usize; 3] {
		return [i+(2*i)+1, 
				i+(2*(i+1)),
				i+(2*(i+1)+1)]
	}

	/// Converts x,y coordinates into a space on the board (1-9).
	fn coords_to_space(coords: (usize, usize)) -> usize {
		return Board::get_row_nums(coords.0)[coords.1];
	}

	/// Converts a space number (1-9) to x,y coordinates in the matrix.
	fn space_to_coords(space: usize) -> (usize, usize) {
		if space % 3 == 0 {
			return ((space/3)-1, 2);
		} else if (space+1) % 3 == 0 {
			return (((space+1)/3)-1, 1);
		} else {
			return (((space+2)/3)-1, 0);
		}
	}

	/// Marks the given space with the current active player's number.
	pub fn set_space(&mut self, space: usize) {
		let coords: (usize, usize) = Board::space_to_coords(space);
		self.board[coords.0][coords.1] = self.active_player;
	}

	/// Returns a vector of free spaces on the board.
	pub fn open_spaces(&self) -> Vec<usize> {

		let mut available: Vec<usize> = Vec::new();

		for (i, row) in self.board.iter().enumerate() {
			
			let row_nums: [usize; 3] = Board::get_row_nums(i);
			if row[0] == 0 {available.push(row_nums[0]);}
			if row[1] == 0 {available.push(row_nums[1]);}
			if row[2] == 0 {available.push(row_nums[2]);}

		}

		return available;
	}

	/// Returns whether or not the given space is occupied.
	pub fn space_taken(&self, space: usize) -> bool {
		let coords: (usize, usize) = Board::space_to_coords(space);
		return self.board[coords.0][coords.1] > 0;
	}

	/// Returns whether or not the board is full.
	pub fn is_full(&self) -> bool {
		for row in self.board.iter() {
			if row.contains(&0) {
				return false;
			}
		}
		return true;
	}

	/// Returns a tuple containing the current state of the game
	/// and the winning move if the game is over.
	/// A `-1` indicates an unfinished game, `0` indicates a draw,
	/// and a `1` or `2` indicates the winning player. If there's 
	/// a winner, an array of the winning spaces is given.
	pub fn is_terminal(&self) -> (i8, [usize; 3]) {

		// check rows and columns for 3 in a row
		for (i, row) in self.board.iter().enumerate() {
			if row[0] == row[1] && row[1] == row[2] && row[0] != 0 {
				let first = Board::coords_to_space((i, 0));
				return (row[0] as i8, [first, first+1, first+2]);
			}
			if self.board[0][i] == self.board[1][i] && 
					self.board[1][i] == self.board[2][i] && self.board[0][i] != 0 {
				let first = Board::coords_to_space((0,i));
				return (self.board[0][i] as i8, [first, first+3, first+6]);
			}
		}

		// check 
		if self.board[0][0] == self.board[1][1] && 
			self.board[1][1] == self.board[2][2] && 
			self.board[0][0] != 0 {
			return (self.board[0][0] as i8, [1, 5, 9]);
		}

		if self.board[2][0] == self.board[1][1] && 
			self.board[1][1] == self.board[0][2] && 
			self.board[2][0] != 0 {
			return (self.board[2][0] as i8, [7, 5, 3]);
		}

		if self.is_full() {
			// if we've gotten here and the board is full,
			// return "0" indicating a draw
			return (0, [0, 0, 0]);
		} else {
			// the game isn't over
			return (-1, [0, 0, 0]);
		}
		
	}

	/// Returns all possible children of the 
	/// current board state.
	pub fn get_children(&self) -> Vec<(usize, Board)> {

		let mut children: Vec<(usize, Board)> = Vec::new();
		let available: Vec<usize> = self.open_spaces();

		// For each available space, create a clone of the 
		// board and fill the space to create the child
		for space in available.iter() {
			let mut child: Board = self.clone();
			child.set_space(*space);
			children.push((*space, child));
		}

		return children;
	}

	/// Converts the current state of the board into a 
	/// heuristic value.
	pub fn heuristic(&self) -> isize {
		let state: i8 = self.is_terminal().0;
		match state {
			0 => -500_000,
			1 => -1_000_000,
			2 => 1_000_000,
			_ => 0
		}
	}

	/// Returns a formatted string with the character that should
	/// appear in the given space. If the space is not occupied, 
	/// the space number will appear in gray. Otherwise, the letter 
	/// of the active player will appear. If the win screen is being 
	/// displayed, it will be green.
	fn convert_char(x: u8, space: usize, green: bool) -> String {
		if x == 0 {
			return format!("{}{}{}", color::Fg(color::Rgb(70,70,70)), 
							space,
							color::Fg(color::Reset));
		} else {
			let xo: char = if x == 1 {'X'} else {'O'};
			if green {
				return format!("{}{}{}{}{}", 
					style::Bold, color::Fg(color::Rgb(0,255,0)),
					xo, color::Fg(color::Reset), style::Reset);
			} else {
				return format!("{}{}{}", style::Bold, xo, style::Reset);
			}
		}
	}

	/// Prints the board with the given spaces highlighted in green
	/// indicating a winner. If three zeros are given, no spaces will 
	/// be green.
	pub fn print_win(&self, win_spaces: [usize; 3]) {

		println!("|---|---|---|");
		for (i, row) in self.board.iter().enumerate() {
			let row_nums: [usize; 3] = Board::get_row_nums(i);
			println!("| {} | {} | {} |{}\n|---|---|---|", 
			Board::convert_char(row[0], row_nums[0], win_spaces.contains(&row_nums[0])), 
			Board::convert_char(row[1], row_nums[1], win_spaces.contains(&row_nums[1])), 
			Board::convert_char(row[2], row_nums[2], win_spaces.contains(&row_nums[2])),
			color::Fg(color::Reset));
		}
	}

	/// Prints the board (with no spaces highlighted green).
	pub fn print(&self) {
		self.print_win([0, 0, 0]);
	}

}