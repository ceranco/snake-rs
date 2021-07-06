//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ggez::{
    self,
    audio::{SoundSource, Source},
    conf::{WindowMode, WindowSetup},
    event::{self, Axis, Button, EventHandler, KeyCode},
    graphics::{
        self, spritebatch::SpriteBatch, Color, DrawParam, Image, Scale, Text, TextFragment,
        DEFAULT_FONT_SCALE,
    },
    input::{self, gamepad::GamepadId},
    mint::Point2,
    timer, Context, ContextBuilder, GameResult,
};
use std::env;
use std::path;
use std::time::{Instant};

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
    sprites: Image,
    eat_sound: Source,
    die_sound: Source,
    background: SpriteBatch,
    update_segment: f32,
}

impl Game {
    /// Helper function to create a new `Game`.
    fn new(ctx: &mut Context) -> GameResult<Self> {
        // load the spritesheet
        let sprites = Image::new(ctx, "/sprites.png")?;

        // load the audio
        let eat_sound = Source::new(ctx, "/eat-sound.ogg")?;
        let mut die_sound = Source::new(ctx, "/die-sound.ogg")?;
        die_sound.set_volume(0.5);

        // generate the background spritebatch
        let mut background = SpriteBatch::new(sprites.clone());
        for x in 0..GRID_SIZE.0 {
            for y in 0..GRID_SIZE.1 {
                background.add(&PositionedSprite::new(
                    Sprite::Grass,
                    GridPosition::new(x, y),
                ));
            }
        }

        Ok(Self {
            snake: Snake::new((1, 0).into()),
            food: Food::new((GRID_SIZE.0 / 2, GRID_SIZE.1 / 2).into()),
            game_over: false,
            last_update: Instant::now(),
            sprites,
            eat_sound,
            die_sound,
            background,
            update_segment: 0.0,
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

    /// Helper function that restarts reinitializes
    /// the `Game` to its starting state.
    ///
    /// note: there is a probably a better and more
    /// efficient way to do this.
    fn restart(&mut self, ctx: &mut Context) {
        let game = Game::new(ctx).expect("Failed to restart the game");
        self.snake = game.snake;
        self.food = game.food;
        self.game_over = game.game_over;
        self.last_update = game.last_update;
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // update the update segment, i.e. the amount of time that
        // has passed from the previous update relative to the amount
        // of time remaining until the next update.
        self.update_segment =
            (Instant::now() - self.last_update).as_millis() as f32 / MILLIS_PER_UPDATE as f32;
        println!("update segment: {}", self.update_segment);
        // we want to update only if the game is not over and enough time
        // has passed since the last update
        if !self.game_over && self.update_segment >= 1.0 {
            if let Some(ate) = self.snake.update(&self.food) {
                match ate {
                    // game over if the snake ate itself
                    Ate::Itself => {
                        self.die_sound.play()?;
                        self.game_over = true;
                    }
                    // if the snake ate the food, we need to change its position
                    // note: we need to add a way grill a random position *without*
                    // a snake segment.
                    Ate::Food => {
                        self.eat_sound.play()?;
                        self.food.set_position(self.generate_food_position())
                    }
                }
            }
            // update the last update time
            self.last_update = Instant::now();
            self.update_segment = 0.0;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(40, 50, 130));

        // draw the game in the following order: background -> snake -> food
        graphics::draw(ctx, &mut self.background, DrawParam::default())?;
        self.snake.draw(ctx, self.update_segment, &mut self.sprites)?;
        self.food.draw(ctx, &mut self.sprites)?;

        // show the game-over screen
        // TODO: I'm pretty sure that creating a new `TextFragment` each time is
        // a horrible horrible thing to do....
        if self.game_over {
            let fragment = TextFragment::new("Game Over! \nPress ENTER or START to play again")
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
        // quit the game
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
        else if self.game_over && keycode == KeyCode::Return {
            self.restart(ctx);
        }
    }

    fn gamepad_button_down_event(&mut self, ctx: &mut Context, btn: Button, _id: GamepadId) {
        // quit the game
        if btn == Button::Select {
            event::quit(ctx);
        }
        // update the direction
        if let Some(direction) = Direction::from_button(btn) {
            // this method may fail if the direction is not orthogonal,
            // but we don't especially care ;)
            let _ = self.snake.set_direction(direction);
        }
        // restart the game
        else if self.game_over && btn == Button::Start {
            self.restart(ctx);
        }
    }

    fn gamepad_axis_event(&mut self, _ctx: &mut Context, axis: Axis, value: f32, _id: GamepadId) {
        if let Some(direction) = Direction::from_axis(axis, value) {
            // this method may fail if the direction is not orthogonal,
            // but we don't especially care ;)
            let _ = self.snake.set_direction(direction);
        }
    }
}

fn main() {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    // create the new context and window with the correct dimensions and title
    let (mut ctx, mut events_loop) = ContextBuilder::new("Snake", "Eran Cohen")
        .window_setup(WindowSetup::default().title("Snake"))
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .add_resource_path(resource_dir)
        .build()
        .unwrap();
    // create a new game
    let mut game = Game::new(&mut ctx).unwrap();

    // run the game
    event::run(&mut ctx, &mut events_loop, &mut game).unwrap();
}
