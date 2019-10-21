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

#[derive(PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn velocity(&self) -> Vector2<i32> {
        match self {
            Direction::Up => Vector2 { x: 0, y: -1 },
            Direction::Down => Vector2 { x: 0, y: 1 },
            Direction::Left => Vector2 { x: -1, y: 0 },
            Direction::Right => Vector2 { x: 1, y: 0 },
        }
    }

    fn orthogonal(&self, other: &Direction) -> bool {
        match self {
            Direction::Up | Direction::Down => {
                other == &Direction::Left || other == &Direction::Right
            }
            Direction::Left | Direction::Right => {
                other == &Direction::Up || other == &Direction::Down
            }
        }
    }

    fn from(key_code: event::KeyCode) -> Option<Self> {
        match key_code {
            event::KeyCode::Up => Some(Direction::Up),
            event::KeyCode::Down => Some(Direction::Down),
            event::KeyCode::Left => Some(Direction::Left),
            event::KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

struct Game {
    grid: (i32, i32),
    cell: (i32, i32),
    update_interval: u16,
    last_update: u128,
    snake: Snake,
    direction: Direction,
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
            direction: Direction::Right,
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
            let velocity = self.direction.velocity();
            let new_position = Point2 {
                x: self.snake.position.x + velocity.x,
                y: self.snake.position.y + velocity.y,
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
        // update the direction
        match Direction::from(keycode) {
            Some(direction) => {
                if self.direction.orthogonal(&direction) {
                    self.direction = direction
                }
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
