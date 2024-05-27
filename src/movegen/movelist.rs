// Marcel Vanthoor
// https://github.com/mvanthoor/rustic/blob/master/src/movegen/movelist.rs

use std::mem;

use super::defs::Move;

// Movelist struct holden the array and counter.
#[derive(Clone, Copy)]
pub struct MoveList {
	list: [Move; 218],
	count: usize,
}

impl MoveList {
	// Creates a new move list. YES, I know that the use of MaybeUninit
	// directly followed by assume_init() is, officially speaking,
	// incorrect because it DOES create a memory block with uninitialized
	// variables. The memory doesn't need to be initialized, because the
	// next step after creating the move list will always be to generate
	// moves and store them in the list beginning at index 0. This would
	// overwrite the initialization and make it useless. Initializing the
	// move list with 0's massively slows down the program. Maybe in the
	// future, I'll rewrite the move generator function to create and fill
	// in the list by itself, without taking a reference to an empty list.
	pub fn new() -> Self {
		Self {
			list: unsafe {
				let block = mem::MaybeUninit::uninit();
				block.assume_init()
			},
			count: 0,
		}
	}

	// Used to store a move in the move list.
	pub fn push(&mut self, m: Move) {
		self.list[self.count] = m;
		self.count += 1;
	}

	// Returns the number of moves in the move list.
	pub fn len(&self) -> usize {
		self.count
	}

	// Return the move at the given index. If out of bounds, the program crashes.
	pub fn get_move(&self, index: usize) -> Move {
		self.list[index]
	}
}
