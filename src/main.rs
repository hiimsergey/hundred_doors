use std::{process, thread, time::Duration};
use clap::Parser;
use ncurses::*;

#[derive(Parser)]
#[command(author, version)]
#[command(arg_required_else_help(true))]
/// A tool for calculating and visualizing the "100 Doors" problem.
struct Args {
	/// How many doors there are
	#[arg(default_value_t = 100)]
	number: usize,

	/// Whether there should be a graphical representation of these doors
	#[arg(short, long)]
	draw: bool,

	/// Whether to explain the problem
	#[arg(short, long)]
	explain: bool,

	/// How many milliseconds to wait before the next wave
	#[arg(long, default_value_t = 0)]
	twave: u64,

	/// How many milliseconds to wait between doors
	#[arg(long, default_value_t = 0)]
	tdoor: u64
}

const EXPLANATION: &str = "Imagine a hundred closed doors in a corridor.
You go and open every one of them.
Then, you open every second one. (If the door was open, then just close it.)
Then, every third. Then, every fourth and every fifth and so on until you only touch the 100th door.
You go and open every one of them.

With this program you can set a custom number of doors and have the total number
and their respective indices printed and/or visualized.";

fn main() {
    let args = Args::parse();

	if args.explain {
		println!("{EXPLANATION}");
		process::exit(0);
	}

	let mut doors: Vec<bool> = vec![false; args.number];

	if args.draw {
		draw(&mut doors, args);
	} else {
		for wave in 0..args.number {
			for i in (wave..args.number).step_by(wave + 1) {
				doors[i] = !doors[i];
			}
		}
	}

	println!("{}", doors.iter().filter(|d| **d).count());
}

fn draw(doors: &mut [bool], args: Args) {
    initscr();
    raw();
    keypad(stdscr(), true);
    timeout(0);

	// Register colors, red and green
	start_color();
	init_pair(1, COLOR_RED, COLOR_BLACK);
	init_pair(2, COLOR_GREEN, COLOR_BLACK);

	for wave in 0..args.number {
		for door in &*doors {
			// Key codes of Ctrl-C, Esc and 'q'
			if [3, 27, 113].contains(&getch()) { process::exit(2); }

			addstr(if *door {
				attron(COLOR_PAIR(1));
				"open "
			} else {
				attron(COLOR_PAIR(2));
				"shut "
			});
			refresh();
			thread::sleep(Duration::from_millis(args.tdoor));
		}

		// Door toggling part
		for i in (wave..args.number).filter(|i| i % (wave + 1) == 0) {
			doors[i] = !doors[i];
		}

		thread::sleep(Duration::from_millis(args.twave));
		clear();
	}

	attroff(COLOR_PAIR(1));
	attroff(COLOR_PAIR(2));
	
    endwin();
}