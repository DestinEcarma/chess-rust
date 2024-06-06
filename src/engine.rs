use std::{io, time::Instant};

use crate::benchmark::Benchmark;

pub struct Engine;
impl Engine {
	pub fn start() {
		let mut benchmark = Benchmark::default();

		loop {
			let mut input = Default::default();

			io::stdin()
				.read_line(&mut input)
				.expect("Failure to read line!");

			let input_string: String = match input.trim().parse() {
				Ok(string) => string,
				Err(_) => continue,
			};

			let tokens: Vec<&str> = input_string.split(' ').collect();

			match tokens[0] {
				"perft" => Self::perft(&mut benchmark, tokens[1]),
				"move" => Self::make_move(&mut benchmark, tokens[1]),
				"fen" => Self::fen(&mut benchmark, tokens),
				"undo" => Self::unmake(&mut benchmark),
				"display" => Self::display(&benchmark),
				_ => println!("Invalid command!"),
			}

			println!();
		}
	}
}

impl Engine {
	fn display(benchmark: &Benchmark) {
		println!("{}", benchmark.board);
	}

	fn perft(benchmark: &mut Benchmark, token: &str) {
		let depth: usize = match token.parse() {
			Ok(num) => {
				if num > 0 {
					num
				} else {
					println!("Invalid argument, must be greater than 0!");
					return;
				}
			}
			Err(_) => {
				println!("Invalid argument, must be a type of number!");
				return;
			}
		};

		let start = Instant::now();
		let nodes = benchmark.perft(depth);
		let elapsed = start.elapsed().as_millis();

		let nodes_per_seconds = match nodes.checked_div(match elapsed.checked_div(1000) {
			Some(num) => num as usize,
			None => 1,
		}) {
			Some(nodes) => nodes,
			None => nodes,
		};

		println!("\nTotal Time (ms)\t: {elapsed}");
		println!("Nodes searched\t: {nodes}");
		println!("Nodes/second\t: {nodes_per_seconds}");
	}

	fn fen(benchmark: &mut Benchmark, tokens: Vec<&str>) {
		if tokens.len() < 4 {
			println!("Invalid argument, argument is incomplete!");
			return;
		}

		let pieces = tokens[1];
		let turn = tokens[2];
		let castle_rights = tokens[3];
		let en_passant = tokens[4];

		match turn {
			"W" | "w" | "B" | "b" => (),
			_ => {
				println!("Invalid argument, character not recognized, got: {turn}");
				return;
			}
		}

		match castle_rights {
			"-" => (),
			_ => {
				for ch in castle_rights.chars() {
					match ch {
						'K' | 'Q' | 'k' | 'q' => (),
						_ => {
							println!("Invalid argument, character not recognized, got: {ch}");
							return;
						}
					}
				}
			}
		}

		match en_passant {
			#[rustfmt::skip]
			"a3" | "b3" | "c3" | "d3" | "e3" | "f3" | "g3" | "h3" |
			"A3" | "B3" | "C3" | "D3" | "E3" | "F3" | "G3" | "H3" |
			"A6" | "B6" | "C6" | "D6" | "E6" | "F6" | "G6" | "H6" |
			"a6" | "b6" | "c6" | "d6" | "e6" | "f6" | "g6" | "h6" |
			"-"=> (),
			_ => {
				println!("Invalid argument, notation not recognized, got: {en_passant}");
				return;
			}
		}

		benchmark.set_fen([pieces, turn, castle_rights, en_passant].join(" ").as_str())
	}

	fn make_move(benchmark: &mut Benchmark, token: &str) {
		match benchmark.play_move(token) {
			true => println!("{}", benchmark.board),
			false => println!("Invalid argument, move not found!"),
		}
	}

	fn unmake(benchmark: &mut Benchmark) {
		benchmark.board.unmake();
	}
}
