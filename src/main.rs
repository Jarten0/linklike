use bevy_reflect::Reflect;
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color};
use ggez::Context;
use glam::Vec2;
use level::Level;
use player::Protag;

pub mod collision;
pub mod enemies;
pub mod item;
pub mod level;
pub mod player;
pub mod sword;

fn main() {
    let (mut ctx, event) = ggez::ContextBuilder::new("linklike", "jarten")
        .build()
        .expect("could not build :(");

    let state = Game::new(&mut ctx);

    ggez::event::run(ctx, event, state);
}

struct Game {
    level: Level,
}

impl Game {
    fn new(ctx: &mut ggez::Context) -> Self {
        let level = Level::new(ctx);
        Self { level }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        self.level.update(ctx)
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = Canvas::from_frame(&ctx.gfx, Color::BLACK);

        self.level.draw(ctx, &mut canvas)?;

        canvas.finish(&mut ctx.gfx)
    }
}

#[derive(Debug, Clone, Copy, Reflect, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Explicit cast to [`glam::Vec2`] since [`Into`] is too inconvenient to use explicitly
    pub fn to_vec(self) -> Vec2 {
        self.into()
    }

    /// Gets an angle value
    pub fn to_angle(self) -> f32 {
        f32::to_degrees(self.to_vec().angle_between(Direction::Right.to_vec()))
    }
}

impl From<Direction> for glam::Vec2 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Vec2::NEG_Y,
            Direction::Down => Vec2::Y,
            Direction::Left => Vec2::NEG_X,
            Direction::Right => Vec2::X,
        }
    }
}

impl From<&Direction> for glam::Vec2 {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Up => Vec2::NEG_Y,
            Direction::Down => Vec2::Y,
            Direction::Left => Vec2::X,
            Direction::Right => Vec2::NEG_X,
        }
    }
}
