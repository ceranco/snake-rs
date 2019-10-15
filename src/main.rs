use ggez::conf::WindowSetup;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Mesh, Rect};
use ggez::mint::Point2;
use ggez::{Context, ContextBuilder, GameResult};

struct Snake {
    mesh: Mesh,
    position: (usize, usize),
}

impl Snake {
    fn draw(&mut self, ctx: &mut Context, cell_size: &(usize, usize)) -> GameResult {
        let point = Point2 {
            x: (self.position.0 * cell_size.0) as f32,
            y: (self.position.1 * cell_size.1) as f32,
        };

        graphics::draw(ctx, &mut self.mesh, (point,))?;
        Ok(())
    }
}

struct Game {
    grid: (usize, usize),
    cell: (usize, usize),
    update_interval: u16,

    snake: Snake,
}

impl Game {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let cell = (64usize, 64usize);
        let grid = (32usize, 32usize);

        let snake_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, cell.0 as f32, cell.1 as f32),
            graphics::WHITE,
        )?;
        Ok(Self {
            snake: Snake {
                mesh: snake_mesh,
                position: (0, 0),
            },
            cell: cell,
            grid: grid,
            update_interval: 1000,
        })
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(0, 30, 120));
        self.snake.draw(ctx, &self.cell)?;
        graphics::present(ctx)?;
        Ok(())
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
