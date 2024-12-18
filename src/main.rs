use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color};
use ggez::Context;
use glam::Vec2;
use level::{Entity, Level};
use player::Protag;

pub mod ecs;
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

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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
