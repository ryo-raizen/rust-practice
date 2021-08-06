use rand::Rng;
use std::io;

const INPUT_MIN_NUMBER: i8 = 1;
const INPUT_MAX_NUMBER: i8 = 9;

fn main() {
	let mut board_values: [i8; 9] = Default::default();
	println!("### Start ###");
	loop {
		draw_board(board_values.clone());
		println!("Please input number (1 to 9).");
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");
		let input: u32 = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("### Error! ###");
				continue
			},
		};
		if input < INPUT_MIN_NUMBER as u32 {
			println!("### Too small! ###");
			continue
		} else if input > INPUT_MAX_NUMBER as u32 {
			println!("### Too big! ###");
			continue
		} else if (board_values[(input - 1) as usize] as i8) > 0 {
			println!("### Already selected! ###");
			continue
		}

		board_values[(input - 1) as usize] = 1;
		if check_game_over(board_values.clone(), 1) {
			draw_board(board_values.clone());
			println!("### You win! ###");
			break;
		} else if count_board_space(board_values.clone()) < 1 {
			draw_board(board_values.clone());
			println!("### Draw! ###");
			break;
		}

		let mut rand_number: i8 = rand::thread_rng().gen_range(0, count_board_space(board_values.clone()));

		for i in 0..INPUT_MAX_NUMBER {
			if (board_values[i as usize] as i8) > 0 {
				rand_number += 1;
			}
			if rand_number == i {
				board_values[i as usize] = 2;
				break;
			}
		}
		if check_game_over(board_values.clone(), 2) {
			draw_board(board_values.clone());
			println!("### You lose! ###");
			break;
		}

		println!("### Opponent selected {}. ###", rand_number + 1);
	}
}

fn draw_board(values: [i8; 9]) {
	let mut marks: Vec<char> = "123456789".chars().collect();
	for i in 0..(values.len()) {
		match values[i] {
			1 => marks[i] = 'O',
			2 => marks[i] = 'X',
			_ => {},
		}
	}
	println!("=============\n\
		 | {} | {} | {} |\n\
		  =============\n\
		 | {} | {} | {} |\n\
		  =============\n\
		 | {} | {} | {} |\n\
		  =============",
		marks[0].to_string(), marks[1].to_string(), marks[2].to_string(),
		marks[3].to_string(), marks[4].to_string(), marks[5].to_string(),
		marks[6].to_string(), marks[7].to_string(), marks[8].to_string(),
		);
}

fn count_board_space(values: [i8; 9]) -> i8 {
	values.iter().filter(|value| **value == 0).count() as i8
}

fn check_game_over(values: [i8; 9], value: i8) -> bool {
	(values[0] == value && values[1] == value && values[2] == value) ||
	(values[3] == value && values[4] == value && values[5] == value) ||
	(values[6] == value && values[7] == value && values[8] == value) ||
	(values[0] == value && values[3] == value && values[6] == value) ||
	(values[1] == value && values[4] == value && values[7] == value) ||
	(values[2] == value && values[5] == value && values[8] == value) ||
	(values[0] == value && values[4] == value && values[8] == value) ||
	(values[2] == value && values[4] == value && values[6] == value)
}
