use super::block::Block;
use super::button::Button;
use super::geometry::*;
use super::graphics::*;
use super::snake::Snake;

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

pub trait Randomizer {
    fn random_between(&self, lower: u32, higher: u32) -> u32;
}

pub struct Game {
    snake: Snake,
    food_exists: bool,
    food_x: u32,
    food_y: u32,
    size: Size,
    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(size: Size) -> Game {
        return Game {
            snake: Snake::new(Point { x: 4, y: 2 }),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
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

        if !self.food_exists {
            self.add_food(randomizer);
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_eaten(&mut self) {
        let head = self.snake.head_position();
        if self.food_exists && self.food_x == head.x && self.food_y == head.y {
            self.food_exists = false;
            self.snake.restore_tail();
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
        let mut new_x = randomizer.random_between(1, self.size.width - 1);
        let mut new_y = randomizer.random_between(1, self.size.height - 1);

        while self.snake.is_overlaping(&Point { x: new_x, y: new_y }) {
            new_x = randomizer.random_between(1, self.size.width - 1);
            new_y = randomizer.random_between(1, self.size.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
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
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }

    pub fn draw(&self) -> Vec<Block> {
        let mut blocks = Vec::new();

        let mut snake = self.snake.draw();
        blocks.append(&mut snake);

        if self.food_exists {
            let food = Block::new(self.food_x, self.food_y, 1, 1, FOOD_COLOR);
            blocks.push(food);
        }
        blocks.push(Block::new(0, 0, self.size.height, 1, BORDER_COLOR));
        blocks.push(Block::new(
            self.size.width - 1,
            0,
            self.size.height,
            1,
            BORDER_COLOR,
        ));
        blocks.push(Block::new(0, 0, 1, self.size.width, BORDER_COLOR));
        blocks.push(Block::new(
            0,
            self.size.height - 1,
            1,
            self.size.width,
            BORDER_COLOR,
        ));

        if self.game_over {
            blocks.push(Block::new(
                0,
                0,
                self.size.height,
                self.size.width,
                GAME_OVER_COLOR,
            ));
        }
        return blocks;
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
        game.food_exists = false;
        let randomizer = TestRandomizer {
            number_completely_random: 10,
        };
        game.add_food(&randomizer);
        assert!(game.food_exists);
        assert_eq!(game.food_x, randomizer.number_completely_random);
        assert_eq!(game.food_y, randomizer.number_completely_random);
    }
    #[test]
    fn test_has_not_eaten_with_food_out_of_range() {
        let mut game = new_game();
        // Set food far away from snake
        game.food_x = game.snake.head_position().x + 10;
        game.food_y = game.snake.head_position().y + 10;
        let snake_length = game.snake.len();
        // One step to the right
        game.update_snake(Some(Direction::Right));
        let snake_new_length = game.snake.len();
        // Food was not eaten
        assert!(game.food_exists);
        assert_eq!(snake_length, snake_new_length);
    }
    #[test]
    fn test_has_eaten_with_food_on_range() {
        let mut game = new_game();
        // Set food at the right of the snake
        game.food_x = game.snake.head_position().x + 1;
        game.food_y = game.snake.head_position().y;
        let snake_length = game.snake.len();
        // One step at the right
        game.update_snake(Some(Direction::Right));
        let snake_new_length = game.snake.len();
        // Food has been eaten
        assert!(game.food_exists == false);
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
        game.food_x = game.snake.head_position().x + 1;
        game.food_y = game.snake.head_position().y;

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

    fn new_game() -> Game {
        return Game::new(Size {
            height: 20,
            width: 20,
        });
    }
}
