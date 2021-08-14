use rand::Rng;
use std::process::Command;
use std::{thread, time};

const WIDTH: u32 = 52;
const HEIGHT: u32 = 32;
const INTERVAL: u64 = 500;
const INIT_RATE: u32 = 10;

fn main() {
	let mut generation: u32 = 0;
	let sleep_millis = time::Duration::from_millis(INTERVAL);
	let mut board_values = init_board();
	loop {
		generation += 1;
		clear();
		draw_board(board_values.clone());
		board_values = generate_next(board_values.clone());
		println!("Generation {}.", generation);
		println!("Press Ctrl+C to Quit.");
		thread::sleep(sleep_millis);
	}
}

fn init_board() -> [[bool;WIDTH as usize]; HEIGHT as usize] {
	let mut initial_values = [[false;WIDTH as usize]; HEIGHT as usize];
	for y in 1..(HEIGHT - 1) {
		for x in 1..(WIDTH - 1) {
			initial_values[y as usize][x as usize] = rand::thread_rng().gen_range(0, INIT_RATE) == 0;
		}
	}
	initial_values
}

fn draw_board(values: [[bool;WIDTH as usize]; HEIGHT as usize]) {
	for y in 0..HEIGHT {
		for x in 0..WIDTH {
			if values[y as usize][x as usize] {
				print!("■");
			} else {
				print!(" ");
			}
		}
		print!("\n");
	}
}

fn generate_next(values: [[bool;WIDTH as usize]; HEIGHT as usize]) -> [[bool;WIDTH as usize]; HEIGHT as usize] {
	let mut next_values = [[false;WIDTH as usize]; HEIGHT as usize];
	for y in 1..(HEIGHT - 1) {
		for x in 1..(WIDTH - 1) {
			let mut count: u32 = 0;
			if values[(y - 1) as usize][(x - 1) as usize] {
				count += 1;
			}
			if values[(y - 1) as usize][(x + 1) as usize] {
				count += 1;
			}
			if values[(y + 1) as usize][(x - 1) as usize] {
				count += 1;
			}
			if values[(y + 1) as usize][(x + 1) as usize] {
				count += 1;
			}
			if values[(y - 1) as usize][x as usize] {
				count += 1;
			}
			if values[(y + 1) as usize][x as usize] {
				count += 1;
			}
			if values[y as usize][(x - 1) as usize] {
				count += 1;
			}
			if values[y as usize][(x + 1) as usize] {
				count += 1;
			}
			if values[y as usize][x as usize] && (count == 2 || count == 3) {
				next_values[y as usize][x as usize] = true;
			} else if !values[y as usize][x as usize] && count == 3 {
				next_values[y as usize][x as usize] = true;
			}
		}
	}
	next_values
}

fn clear() {
	let output = Command::new("clear").output().unwrap_or_else(|e| {
			panic!("failed to execute process: {}", e)
			});
	print!("{}", String::from_utf8_lossy(&output.stdout));
}
