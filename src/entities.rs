use crate::primitives::*;
use ggez::{
    graphics::{self, DrawParam, Image},
    Context, GameResult,
};
use std::collections::VecDeque;

/// Contains all the information needed to describe
/// the state of the snake itself.
pub struct Snake {
    head: PositionedSprite,
    body: VecDeque<PositionedSprite>,
    tail: PositionedSprite,
    direction: Direction,
    previous_direction: Direction,
    next_direction: Option<Direction>,
}

impl Snake {
    /// Creates a new `Snake` with one head segment at the
    /// given position and one `Tail` segment behind it (direction is right).
    pub fn new(position: GridPosition) -> Self {
        let head = PositionedSprite::new(Sprite::Head(Direction::Right), position);
        let body = VecDeque::new();
        let tail = PositionedSprite::new(
            Sprite::Tail(Direction::Right),
            GridPosition::new_from_move(position, Direction::Left),
        );
        Self {
            head,
            body,
            tail,
            direction: Direction::Right,
            previous_direction: Direction::Right,
            next_direction: None,
        }
    }

    /// Sets the direction of the snake.
    ///
    /// If the direction is invalid, returns a `SnakeError::LogicError`.
    pub fn set_direction(&mut self, direction: Direction) -> SnakeResult<()> {
        let is_valid = self.direction.inverse() != direction;
        if is_valid {
            if self.previous_direction == self.direction {
                self.direction = direction;
            } else {
                self.next_direction = Some(direction);
            }
            Ok(())
        } else {
            Err(SnakeError::LogicError(
                "Can only update direction if it is orthogonal to previous direction".to_owned(),
            ))
        }
    }

    /// Returns a `Vec` of ***all*** of the segments of the
    /// snake (including the head).
    ///
    /// This is useful to generate a new position for the
    /// food and check that the snake isn't already there.
    pub fn segments(&self) -> Vec<GridPosition> {
        let mut vec: Vec<GridPosition> = self.body.iter().map(|segment| segment.position).collect();
        vec.push(self.head.position);
        vec
    }

    /// Helper function that checks if the `Snake`
    /// is eating the `Food` in its current state.
    fn eats_food(&self, food: &Food) -> bool {
        self.head.position == food.position()
    }

    /// Helper function that checks if the `Snake`
    /// is eating itself in its current state.
    fn eats_self(&self) -> bool {
        self.body
            .iter()
            .any(|segment| segment.position == self.head.position)
    }

    /// Updates the state of the `Snake`.
    pub fn update(&mut self, food: &Food) -> Option<Ate> {
        // move in the set direction
        let new_head = PositionedSprite::new(
            Sprite::Head(self.direction),
            GridPosition::new_from_move(self.head.position, self.direction),
        );

        // push the current head-position to the body
        // as a segment from `previous_direction` -> `direction`
        // and update it to the new one
        self.body.push_front(PositionedSprite::new(
            Sprite::Segment(self.previous_direction.inverse(), self.direction),
            self.head.position,
        ));
        self.head = new_head;

        // update the "direction cache"
        self.previous_direction = self.direction;
        if let Some(next_direction) = self.next_direction {
            self.direction = next_direction;
            self.next_direction = None;
        }

        // check if the snake is eating something
        if self.eats_food(food) {
            Some(Ate::Food)
        } else {
            // if the snake didn't eat food, move the last body segment
            // to the tail to create the illusion of movement
            let mut new_tail = self.body.pop_back().expect("The body was empty?!");
            let tail_direction = match new_tail.sprite {
                Sprite::Segment(_, tgt) => tgt,
                _ => panic!("The body sprite wasn't `Sprite::Segment`."),
            };
            new_tail.sprite = Sprite::Tail(tail_direction);
            self.tail = new_tail;

            if self.eats_self() {
                Some(Ate::Itself)
            } else {
                None
            }
        }
    }

    /// Draws the `Snake` to the screen in its current state.
    pub fn draw(&mut self, ctx: &mut Context, sprites: &mut Image) -> GameResult {
        // draw the tail
        let mut param: DrawParam = (&self.tail).into();
        graphics::draw(ctx, sprites, param)?;

        // draw the body
        for segment in &self.body {
            param = segment.into();
            graphics::draw(ctx, sprites, param)?;
        }

        // draw the head last to show it ontop anything else
        param = (&self.head).into();
        graphics::draw(ctx, sprites, param)?;
        Ok(())
    }
}

/// Represents a piece of food that the `Snake` can eat.
pub struct Food {
    segment: PositionedSprite,
}

impl Food {
    pub fn new(position: GridPosition) -> Self {
        Self {
            segment: PositionedSprite::new(Sprite::Rabit, position),
        }
    }

    pub fn position(&self) -> GridPosition {
        self.segment.position
    }

    pub fn set_position(&mut self, position: GridPosition) {
        self.segment.position = position;
    }

    pub fn draw(&mut self, ctx: &mut Context, sprites: &mut Image) -> GameResult {
        let param: DrawParam = (&self.segment).into();
        graphics::draw(ctx, sprites, param)?;
        Ok(())
    }
}
