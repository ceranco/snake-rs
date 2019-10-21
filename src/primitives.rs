use ggez::event;
use ggez::graphics;
use ggez::graphics::Mesh;
use ggez::mint::{Point2, Vector2};
use ggez::{Context, GameResult};

#[derive(Debug, Clone)]
pub enum SnakeError {
    LogicError(String),
}
pub type SnakeResult<T = ()> = Result<T, SnakeError>;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn velocity(&self) -> Vector2<i32> {
        match self {
            Direction::Up => Vector2 { x: 0, y: -1 },
            Direction::Down => Vector2 { x: 0, y: 1 },
            Direction::Left => Vector2 { x: -1, y: 0 },
            Direction::Right => Vector2 { x: 1, y: 0 },
        }
    }

    pub fn orthogonal(&self, other: Direction) -> bool {
        match self {
            Direction::Up | Direction::Down => {
                other == Direction::Left || other == Direction::Right
            }
            Direction::Left | Direction::Right => {
                other == Direction::Up || other == Direction::Down
            }
        }
    }

    pub fn from(key_code: event::KeyCode) -> Option<Self> {
        match key_code {
            event::KeyCode::Up => Some(Direction::Up),
            event::KeyCode::Down => Some(Direction::Down),
            event::KeyCode::Left => Some(Direction::Left),
            event::KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

pub struct Snake {
    mesh: Mesh,
    direction: Direction,
    position: Point2<i32>,
}

impl Snake {
    pub fn new(mesh: Mesh, position: Point2<i32>, direction: Direction) -> Self {
        Self {
            mesh: mesh,
            direction: direction,
            position: position,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) -> SnakeResult<()> {
        if self.direction.orthogonal(direction) {
            self.direction = direction;
            Ok(())
        } else {
            Err(SnakeError::LogicError(
                "Can only update direction if it is orthogonal to previous direction".to_owned(),
            ))
        }
    }

    pub fn update(&mut self) {
        let velocity = self.direction.velocity();
        self.position = Point2 {
            x: self.position.x + velocity.x,
            y: self.position.y + velocity.y,
        };
    }

    pub fn draw(&mut self, ctx: &mut Context, cell_size: (i32, i32)) -> GameResult {
        let point = Point2 {
            x: (self.position.x * cell_size.0) as f32,
            y: (self.position.y * cell_size.1) as f32,
        };
        graphics::draw(ctx, &mut self.mesh, (point,))?;
        Ok(())
    }
}
