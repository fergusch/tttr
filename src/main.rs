extern crate termion;
extern crate rand;
extern crate clap;
extern crate ctrlc;

mod player;
mod board;
mod game;

use game::Game;
use player::{*};
use termion::style;

fn main() {

	// load cli args from cli.yml
	let yaml = clap::load_yaml!("../cli.yml");
	let args = clap::App::from_yaml(yaml).get_matches();

	// get the gameplay mode specified (defaults to "easy")
	let mode = args.value_of("mode").unwrap_or("hard");

	// set Ctrl-C handler
	ctrlc::set_handler(move || {
        println!("\n\nQuitters never win.\n");
		std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

	// start game based on mode
	match mode {
		"easy" => Game::new(Human::new(), RandomAI::new()),
		"hard" => Game::new(Human::new(), MinimaxAI::new(8)),
		"2p"   => Game::new(Human::new(), Human::new()),
		"sim"  => Game::new(MinimaxAI::new(8), MinimaxAI::new(4)),
		_      => println!("{}Invalid mode given.{} Use the --help flag \
					to see valid game modes.", style::Bold, style::Reset)
	}

}