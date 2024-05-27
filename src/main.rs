#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

use benchmark::Benchmark;
use board::Board;
use engine::Engine;

mod benchmark;
mod bitboard;
mod board;
mod castle_right;
mod color;
mod engine;
mod magic;
mod movegen;
mod piece;
mod square;

fn main() {
	Engine::start();
}
