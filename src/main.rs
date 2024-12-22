use core::f32;

use assets::StaticAssets;
use bevy_reflect::Reflect;
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color};
use ggez::Context;
use glam::Vec2;
use level::Level;
use protag::Protag;

pub mod assets;
pub mod collision;
pub mod enemies;
pub mod get;
pub mod level;
pub mod protag;

fn main() {
    let (mut ctx, event) = ggez::ContextBuilder::new("linklike", "jarten")
        .build()
        .expect("could not build :(");

    // Allocated onto the stack afaik
    let mut static_assets: StaticAssets = StaticAssets::new();
    Level::initialize_assets(&mut static_assets);

    // Box::from_raw can work with stack references as well???
    let leak = Box::leak::<'static>(Box::new(static_assets));

    let state = Game::new(&mut ctx, leak);

    ggez::event::run(ctx, event, state);
}

struct Game {
    level: Level,
    /// After [`Game::new`], this is permanently borrowed to Level.
    static_assets: &'static StaticAssets,
}

impl Game {
    fn new(ctx: &mut ggez::Context, static_assets: &'static StaticAssets) -> Self {
        Self {
            level: Level::new(ctx, &static_assets),
            static_assets,
        }
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

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq, Hash)]
pub enum Direction {
    Right = 0,
    Up = 1,
    Left = 2,
    Down = 3,
}

impl Direction {
    /// Explicit cast to [`glam::Vec2`] since [`Into`] is too inconvenient to use explicitly
    pub const fn to_vec(self) -> Vec2 {
        match self {
            Direction::Up => Vec2::NEG_Y,
            Direction::Down => Vec2::Y,
            Direction::Left => Vec2::NEG_X,
            Direction::Right => Vec2::X,
        }
    }

    /// Gets an angle value
    pub const fn to_angle(self) -> f32 {
        // f32::to_degrees(self.to_vec().angle_between(Direction::Right.to_vec()))
        match self {
            Direction::Up => f32::consts::PI / 2.,
            Direction::Down => -f32::consts::PI / 2.,
            Direction::Left => f32::consts::PI,
            Direction::Right => 0.0,
        }
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
