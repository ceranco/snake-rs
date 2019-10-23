use ggez::conf::WindowSetup;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Mesh, Rect};
use ggez::input;
use ggez::mint::Point2;
use ggez::{Context, ContextBuilder, GameResult};

use rand::{self, Rng};

mod primitives;
use primitives::*;

struct Game {
    grid: (i32, i32),
    cell: (i32, i32),
    update_interval: u16,
    last_update: u128,
    snake: Snake,
    food: Food,
}

impl Game {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let window_size = ggez::graphics::screen_coordinates(ctx);
        let cell = (32, 32);
        let grid = (
            (window_size.w / cell.0 as f32) as i32,
            (window_size.h / cell.1 as f32) as i32,
        );

        let snake_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, cell.0 as f32, cell.1 as f32),
            graphics::WHITE,
        )?;
        let food_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, cell.0 as f32, cell.1 as f32),
            graphics::Color::from_rgb(200, 100, 0),
        )?;

        Ok(Self {
            snake: Snake::new(snake_mesh, Point2 { x: 0, y: 0 }, Direction::Right),
            food: Food::new(
                food_mesh,
                Point2 {
                    x: grid.0 / 2,
                    y: grid.1 / 2,
                },
            ),
            cell: cell,
            grid: grid,
            update_interval: 100,
            last_update: 0,
        })
    }

    fn gen_food_position(&self) -> Point2<i32> {
        let mut new_food_position = Point2 {
            x: rand::thread_rng().gen_range(0, self.grid.0),
            y: rand::thread_rng().gen_range(0, self.grid.1),
        };
        while self.snake.points().contains(&new_food_position) {
            println!("Ping");
            new_food_position = Point2 {
                x: rand::thread_rng().gen_range(0, self.grid.0),
                y: rand::thread_rng().gen_range(0, self.grid.1),
            };
        }

        new_food_position
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let current_time = ggez::timer::time_since_start(ctx).as_millis();
        let delta = (current_time - self.last_update) as u16;
        if delta >= self.update_interval {
            let position = self.snake.position();
            let velocity = self.snake.direction().velocity();

            let next_position = Point2 {
                x: position.x + velocity.x,
                y: position.y + velocity.y,
            };
            let food_position = self.food.position();

            let grow = next_position == food_position;
            self.snake.update(grow);
            if grow {
                self.food.set_position(self.gen_food_position());
            }
            self.last_update = current_time;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(40, 50, 130));
        self.snake.draw(ctx, self.cell)?;
        self.food.draw(ctx, self.cell)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: input::keyboard::KeyMods,
        _repeat: bool,
    ) {
        if keycode == event::KeyCode::Escape {
            event::quit(ctx);
        }
        // update the direction
        match Direction::from(keycode) {
            Some(direction) => {
                // this method may fail if the direction is not orthogonal,
                // but we don't especially care ;)
                let _ = self.snake.set_direction(direction);
            }
            _ => (),
        }
    }
}

fn main() {
    let (mut ctx, mut events_loop) = ContextBuilder::new("Snake", "Eran Cohen")
        .window_setup(WindowSetup::default().title("Snake"))
        .build()
        .unwrap();
    let mut game = Game::new(&mut ctx).unwrap();

    event::run(&mut ctx, &mut events_loop, &mut game).unwrap();
}
