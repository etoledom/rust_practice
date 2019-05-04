extern crate utilities;
use utilities::geometry::{Point, Rect, Size};
use utilities::graphics::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
	rect: Rect,
	pub color: Color,
}

impl Block {
	pub fn new(x: u32, y: u32, height: u32, width: u32, color: Color) -> Block {
		return Block {
			rect: Rect {
				origin: Point { x, y },
				size: Size { height, width },
			},
			color,
		};
	}

	pub fn size(&self) -> Size {
		return self.rect.size.clone();
	}

	pub fn position(&self) -> Point {
		return self.rect.origin.clone();
	}
}
