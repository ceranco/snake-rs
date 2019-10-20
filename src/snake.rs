use ggez::graphics;
use ggez::graphics::Mesh;
use ggez::mint::Point2;
use ggez::{Context, GameResult};

pub struct Snake {
    pub mesh: Mesh,
    pub position: Point2<i32>,
}

impl Snake {
    pub fn update(&mut self, position: Point2<i32>) -> GameResult {
        self.position = position;
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, cell_size: &(i32, i32)) -> GameResult {
        let point = Point2 {
            x: (self.position.x * cell_size.0) as f32,
            y: (self.position.y * cell_size.1) as f32,
        };
        graphics::draw(ctx, &mut self.mesh, (point,))?;
        Ok(())
    }
}
