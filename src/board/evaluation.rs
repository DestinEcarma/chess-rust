use crate::color::Colors;

use super::Board;

pub type Score = i16;

impl Board {
	pub fn evaluate(&self) -> Score {
		let color = self.color();
		let w_material = self.game_state.material[Colors::WHITE] as Score;
		let b_material = self.game_state.material[Colors::BLACK] as Score;

		let score = w_material - b_material;

		
		
		score
	}
}
