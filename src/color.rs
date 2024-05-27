pub type Color = usize;

pub struct Colors;
impl Colors {
	pub const WHITE: Color = 0;
	pub const BLACK: Color = 1;

	pub const SIZE: usize = 2;

	pub fn from_char(value: char) -> Color {
		if value == value.to_ascii_uppercase() {
			Self::WHITE
		} else {
			Self::BLACK
		}
	}
}
