use ggez::conf::WindowSetup;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Mesh, Rect};
use ggez::input;
use ggez::mint::{Point2, Vector2};
use ggez::{Context, ContextBuilder, GameResult};

mod snake;
use snake::Snake;

struct Game {
    grid: (i32, i32),
    cell: (i32, i32),
    update_interval: u16,
    last_update: u128,
    snake: Snake,
    velocity: Vector2<i32>,
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
            snake: Snake {
                mesh: snake_mesh,
                position: Point2 { x: 0, y: 0 },
            },
            velocity: Vector2 { x: 1, y: 0 },
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
            let new_position = Point2 {
                x: self.snake.position.x + self.velocity.x,
                y: self.snake.position.y + self.velocity.y,
            };
            self.snake.update(new_position)?;

            self.last_update = current_time;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(0, 30, 120));
        self.snake.draw(ctx, &self.cell)?;
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

        self.velocity = match keycode {
            event::KeyCode::Up => Vector2 { x: 0, y: -1},
            event::KeyCode::Down => Vector2 { x: 0, y: 1},
            event::KeyCode::Left => Vector2 { x: -1, y: 0},
            event::KeyCode::Right => Vector2 { x: 1, y: 0},
            _ => self.velocity
        };
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
