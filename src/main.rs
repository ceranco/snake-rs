use ggez::{
    self,
    conf::{WindowMode, WindowSetup},
    event::{self, Axis, Button, EventHandler, KeyCode},
    graphics::{self, Color, DrawMode, Mesh, Rect, Scale, Text, TextFragment, DEFAULT_FONT_SCALE},
    input::{self, gamepad::GamepadId},
    mint::Point2,
    timer, Context, ContextBuilder, GameResult,
};
use std::time::{Duration, Instant};

mod primitives;
use primitives::*;
mod entities;
use entities::*;

/// This is the game state struct that will contain
/// all the needed state (snake, food, game over, ...)
/// and implement the `EventHandler` trait to listen and respond to
/// events.
struct Game {
    game_over: bool,
    snake: Snake,
    food: Food,
    last_update: Instant,
}

impl Game {
    /// Helper function to create a new `Game`.
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let snake_head_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, GRID_CELL_SIZE.0 as f32, GRID_CELL_SIZE.1 as f32),
            graphics::WHITE,
        )?;
        let snake_body_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, GRID_CELL_SIZE.0 as f32, GRID_CELL_SIZE.1 as f32),
            graphics::WHITE,
        )?;
        let food_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, GRID_CELL_SIZE.0 as f32, GRID_CELL_SIZE.1 as f32),
            graphics::Color::from_rgb(200, 100, 0),
        )?;

        Ok(Self {
            snake: Snake::new(snake_body_mesh, snake_head_mesh, (1, 0).into()),
            food: Food::new(food_mesh, (GRID_SIZE.0 / 2, GRID_SIZE.1 / 2).into()),
            game_over: false,
            last_update: Instant::now(),
        })
    }

    /// Helper function that generates a new random
    /// position for the food while ensuring that it
    /// doesn't collide with the snake.
    ///
    /// note: this algorithm ***isn't*** good.
    fn generate_food_position(&self) -> GridPosition {
        let segments = self.snake.segments();
        let mut position = GridPosition::random(GRID_SIZE.0, GRID_SIZE.1);
        while segments.contains(&position) {
            position = GridPosition::random(GRID_SIZE.0, GRID_SIZE.1);
        }
        position
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // we want to update only if the game is not over and enough time
        // has passed since the last update
        if !self.game_over
            && Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE)
        {
            if let Some(ate) = self.snake.update(&self.food) {
                match ate {
                    // game over if the snake ate itself
                    Ate::Itself => self.game_over = true,
                    // if the snake ate the food, we need to change its position
                    // note: we need to add a way grill a random position *without*
                    // a snake segment.
                    Ate::Food => self.food.set_position(self.generate_food_position()),
                }
            }
            // update the last update time
            self.last_update = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(40, 50, 130));
        self.snake.draw(ctx)?;
        self.food.draw(ctx)?;

        if self.game_over {
            let fragment = TextFragment::new("Game Over! \nPress ENTER to play again")
                .scale(Scale::uniform(DEFAULT_FONT_SCALE * 2.0));
            let text = &mut Text::new(fragment);
            let dimensions = text.dimensions(ctx);
            graphics::draw(
                ctx,
                text,
                (Point2 {
                    x: SCREEN_SIZE.0 as f32 * 0.5 - dimensions.0 as f32 * 0.5,
                    y: SCREEN_SIZE.1 as f32 * 0.5 - dimensions.1 as f32,
                },),
            )?;
        }

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: input::keyboard::KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Escape {
            event::quit(ctx);
        }
        // update the direction
        else if let Some(direction) = Direction::from_keycode(keycode) {
            // this method may fail if the direction is not orthogonal,
            // but we don't especially care ;)
            let _ = self.snake.set_direction(direction);
        }
        // restart the game
        else if keycode == KeyCode::Return {
            let game = Game::new(ctx).expect("Failed to restart the game");
            self.snake = game.snake;
            self.food = game.food;
            self.game_over = game.game_over;
            self.last_update = game.last_update;
        }
    }

    fn gamepad_button_down_event(&mut self, _ctx: &mut Context, btn: Button, _id: GamepadId) {
        // update the direction
        if let Some(direction) = Direction::from_button(btn) {
            // this method may fail if the direction is not orthogonal,
            // but we don't especially care ;)
            let _ = self.snake.set_direction(direction);
        }
    }
}

fn main() {
    // create the new context and window with the correct dimensions and title
    let (mut ctx, mut events_loop) = ContextBuilder::new("Snake", "Eran Cohen")
        .window_setup(WindowSetup::default().title("Snake"))
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .unwrap();
    // create a new game
    let mut game = Game::new(&mut ctx).unwrap();

    // run the game
    event::run(&mut ctx, &mut events_loop, &mut game).unwrap();
}
