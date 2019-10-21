use ggez::conf::WindowSetup;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Mesh, Rect};
use ggez::input;
use ggez::mint::Point2;
use ggez::{Context, ContextBuilder, GameResult};

mod primitives;
use primitives::*;

struct Game {
    grid: (i32, i32),
    cell: (i32, i32),
    update_interval: u16,
    last_update: u128,
    snake: Snake,
}

impl Game {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let cell = (64, 64);
        let grid = (32, 32);

        let snake_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, cell.0 as f32, cell.1 as f32),
            graphics::WHITE,
        )?;
        Ok(Self {
            snake: Snake::new(snake_mesh, Point2 { x: 0, y: 0 }, Direction::Right),
            cell: cell,
            grid: grid,
            update_interval: 1000,
            last_update: 0,
        })
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let current_time = ggez::timer::time_since_start(ctx).as_millis();
        let delta = (current_time - self.last_update) as u16;
        if delta >= self.update_interval {
            self.snake.update();
            self.last_update = current_time;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(0, 30, 120));
        self.snake.draw(ctx, self.cell)?;
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
