use super::block::Block;
use super::button::Button;
extern crate utilities;
use super::snake::Snake;
use utilities::geometry::{Direction, Point, Size};
use utilities::graphics::Color;

const FOOD_COLOR: Color = Color {
	red: 0.8,
	green: 0.0,
	blue: 0.0,
	alpha: 1.0,
};

const BORDER_COLOR: Color = Color {
	red: 0.0,
	green: 0.0,
	blue: 0.0,
	alpha: 1.0,
};

const GAME_OVER_COLOR: Color = Color {
	red: 0.9,
	green: 0.0,
	blue: 0.0,
	alpha: 0.5,
};

const MOVING_PERIOD: f64 = 0.2;
const RESTART_TIME: f64 = 1.0;
const INITIAL_FOOD: Point = Point { x: 6, y: 4 };

pub trait Randomizer {
	fn random_between(&self, lower: u32, higher: u32) -> u32;
}

pub struct Game {
	snake: Snake,
	food: Option<Point>,
	size: Size,
	game_over: bool,
	waiting_time: f64,
}

impl Game {
	pub fn new(size: Size) -> Game {
		return Game {
			snake: Snake::new(Point { x: 4, y: 2 }),
			waiting_time: 0.0,
			food: Some(INITIAL_FOOD),
			size,
			game_over: false,
		};
	}

	pub fn button_pressed(&mut self, button: Button) {
		if self.game_over {
			return;
		}

		let direction = match button {
			Button::Up => Some(Direction::Up),
			Button::Down => Some(Direction::Down),
			Button::Left => Some(Direction::Left),
			Button::Right => Some(Direction::Right),
		};

		if direction.unwrap() == self.snake.direction.opposite() {
			return;
		}

		self.update_snake(direction);
	}

	pub fn update(&mut self, delta_time: f64, randomizer: &impl Randomizer) {
		self.waiting_time += delta_time;

		if self.game_over {
			if self.waiting_time > RESTART_TIME {
				self.restart();
			}
			return;
		}

		if self.food.is_none() {
			self.add_food(randomizer);
		}

		if self.waiting_time > MOVING_PERIOD {
			self.update_snake(None);
		}
	}

	fn check_eaten(&mut self) {
		let head = self.snake.head_position();
		if let Some(food) = self.food {
			if food.x == head.x && food.y == head.y {
				self.food = None;
				self.snake.restore_tail();
			}
		}
	}

	fn is_snake_alive(&self, direction: Option<Direction>) -> bool {
		let next = self.snake.next_head(direction);

		if self.snake.is_overlaping(&next) {
			return false;
		}

		return next.x > 0
			&& next.y > 0
			&& next.x < self.size.width - 1
			&& next.y < self.size.height - 1;
	}

	fn add_food(&mut self, randomizer: &impl Randomizer) {
		let mut x = randomizer.random_between(1, self.size.width - 1);
		let mut y = randomizer.random_between(1, self.size.height - 1);

		while self.snake.is_overlaping(&Point { x, y }) {
			x = randomizer.random_between(1, self.size.width - 1);
			y = randomizer.random_between(1, self.size.height - 1);
		}

		self.food = Some(Point { x, y });
	}

	fn update_snake(&mut self, direction: Option<Direction>) {
		if self.is_snake_alive(direction) {
			self.snake.move_forward(direction);
			self.check_eaten();
		} else {
			self.game_over = true;
		}
		self.waiting_time = 0.0;
	}

	fn restart(&mut self) {
		self.snake = Snake::new(Point { x: 2, y: 2 });
		self.waiting_time = 0.0;
		self.food = Some(INITIAL_FOOD);
		self.game_over = false;
	}

	pub fn draw(&self) -> Vec<Block> {
		return [
			self.snake.draw(),
			self.draw_food(),
			self.draw_walls(),
			self.draw_game_over_layer(),
		]
		.concat();
	}

	fn draw_food(&self) -> Vec<Block> {
		if let Some(food) = self.food {
			return vec![Block::new(food.x, food.y, 1, 1, FOOD_COLOR)];
		}
		return vec![];
	}

	fn draw_walls(&self) -> Vec<Block> {
		let (height, width) = (self.size.height, self.size.width);
		return vec![
			Block::new(0, 0, height, 1, BORDER_COLOR),
			Block::new(width - 1, 0, height, 1, BORDER_COLOR),
			Block::new(width - 1, 0, height, 1, BORDER_COLOR),
			Block::new(0, height - 1, 1, width, BORDER_COLOR),
		];
	}

	fn draw_game_over_layer(&self) -> Vec<Block> {
		if self.game_over {
			let (height, width) = (self.size.height, self.size.width);
			return vec![Block::new(0, 0, height, width, GAME_OVER_COLOR)];
		}
		return vec![];
	}
}

#[cfg(test)]
mod game_tests {
	use super::*;

	struct TestRandomizer {
		number_completely_random: u32,
	}

	impl Randomizer for TestRandomizer {
		fn random_between(&self, _lower: u32, _higher: u32) -> u32 {
			return self.number_completely_random;
		}
	}

	#[test]
	fn test_button_press_change_direction() {
		let mut game = new_game();
		game.button_pressed(Button::Down);
		let direction = game.snake.head_direction();
		assert_eq!(direction, Direction::Down);
	}
	#[test]
	fn test_opposite_button_press_does_not_change_direction() {
		let mut game = new_game();
		game.button_pressed(Button::Down);
		assert_eq!(game.snake.head_direction(), Direction::Down);
		game.button_pressed(Button::Up);
		assert_eq!(game.snake.head_direction(), Direction::Down);
	}
	#[test]
	fn test_add_food() {
		let mut game = new_game();
		game.food = None;
		let randomizer = TestRandomizer {
			number_completely_random: 10,
		};
		game.add_food(&randomizer);
		assert!(game.food.is_some());
		assert_eq!(game.food.unwrap().x, randomizer.number_completely_random);
		assert_eq!(game.food.unwrap().y, randomizer.number_completely_random);
	}
	#[test]
	fn test_has_not_eaten_with_food_out_of_range() {
		let mut game = new_game();
		// Set food far away from snake
		let far_away_point = Point {
			x: game.snake.head_position().x + 10,
			y: game.snake.head_position().y + 10,
		};
		game.food = Some(far_away_point);
		let snake_length = game.snake.len();
		// One step to the right
		game.update_snake(Some(Direction::Right));
		let snake_new_length = game.snake.len();
		// Food was not eaten
		assert!(game.food.is_some());
		assert_eq!(snake_length, snake_new_length);
	}
	#[test]
	fn test_has_eaten_with_food_on_range() {
		let mut game = new_game();
		// Set food at the right of the snake
		let near_point = Point {
			x: game.snake.head_position().x + 1,
			y: game.snake.head_position().y,
		};
		game.food = Some(near_point);
		let snake_length = game.snake.len();
		// One step at the right
		game.update_snake(Some(Direction::Right));
		let snake_new_length = game.snake.len();
		// Food has been eaten
		assert!(game.food.is_none());
		assert_eq!(snake_length + 1, snake_new_length);
	}
	#[test]
	fn test_snake_is_alive() {
		let mut game = new_game();
		game.update_snake(Some(Direction::Right));
		assert!(game.is_snake_alive(None));
	}
	#[test]
	fn test_snake_dies_by_overlaping_itself() {
		let mut game = new_game();
		// Set food at the right of the snake to let it grow enough
		let near_point = Point {
			x: game.snake.head_position().x + 1,
			y: game.snake.head_position().y,
		};
		game.food = Some(near_point);

		game.update_snake(Some(Direction::Right));
		game.update_snake(Some(Direction::Down));
		game.update_snake(Some(Direction::Left));
		game.update_snake(Some(Direction::Up));
		assert!(game.game_over);
	}
	#[test]
	fn test_snake_dies_by_crashing_the_wall() {
		let mut game = new_game();
		// Snake starts at y: 2.
		game.update_snake(Some(Direction::Up));
		game.update_snake(Some(Direction::Up));
		assert!(game.game_over);
	}
	#[test]
	fn test_draw_food() {
		let game = new_game();
		let food = game.draw_food();
		assert_eq!(food.len(), 1);
	}
	#[test]
	fn test_food_not_drawn() {
		let mut game = new_game();
		game.food = None;
		let food = game.draw_food();
		assert_eq!(food.len(), 0);
	}
	#[test]
	fn test_draw_walls() {
		let game = new_game();
		let walls = game.draw_walls();
		assert_eq!(walls.len(), 4);
	}
	#[test]
	fn test_draw_game_over_layer() {
		let mut game = new_game();
		game.game_over = true;
		let layer = game.draw_game_over_layer();
		assert_eq!(layer.len(), 1);
	}
	#[test]
	fn test_does_not_draw_game_over_layer() {
		let mut game = new_game();
		game.game_over = false;
		let layer = game.draw_game_over_layer();
		assert_eq!(layer.len(), 0);
	}

	fn new_game() -> Game {
		return Game::new(Size {
			height: 20,
			width: 20,
		});
	}
}
