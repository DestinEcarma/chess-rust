use std::{str::FromStr, time::Instant};

use crate::{board::Board, movegen::MoveGenerator};

#[derive(Default)]
pub struct Benchmark {
	pub board: Board,
	pub movegen: MoveGenerator,
}

impl Benchmark {
	pub fn set_fen(&mut self, fen: &str) {
		self.board = Board::from_str(fen).unwrap();
	}

	pub fn perft(&mut self, depth: usize) {
		let now = Instant::now();

		let mut nodes = 0;

		let list = self.movegen.generate_psuedo(&self.board);

		for i in 0..list.len() {
			let m = list.get_move(i);

			if self.board.make_psuedo_move(m, &self.movegen) {
				let move_nodes = self._perft(depth - 1);
				self.board.unmake();

				nodes += move_nodes;
				println!("{m}: {move_nodes}");
			}
		}

		let elapsed = now.elapsed().as_millis() as f64;
		let nodes_per_seconds = ((nodes * 1000) as f64 / elapsed).floor();

		println!("\nTotal time (ms)\t: {:.0}", elapsed);
		println!("Nodes searched\t: {nodes}");
		println!("Nodes/second\t: {}", nodes_per_seconds);
	}

	#[inline(always)]
	fn _perft(&mut self, depth: usize) -> usize {
		if depth == 0 {
			return 1;
		}

		let mut nodes = 0;

		let list = self.movegen.generate_psuedo(&self.board);

		for m in 0..list.len() {
			if self.board.make_psuedo_move(list.get_move(m), &self.movegen) {
				nodes += self._perft(depth - 1);
				self.board.unmake();
			}
		}

		nodes
	}

	pub fn play_move(&mut self, value: &str) -> bool {
		let list = self.movegen.generate_psuedo(&self.board);

		for i in 0..list.len() {
			let m = list.get_move(i);

			if format!("{m}") == value {
				return self.board.make_psuedo_move(m, &self.movegen);
			}
		}

		false
	}
}
