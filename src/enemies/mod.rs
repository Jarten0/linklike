use crate::collision::HitboxType;
use crate::level::{Level, LevelData};
use basic_enemy::BasicEnemy;
use bevy_reflect::{DynamicTyped, PartialReflect, Reflect};
use ggez::graphics::Canvas;
use ggez::{Context, GameResult};
use glam::Vec2;

pub mod basic_enemy;

pub trait Enemy {
    /// The enemy will be initialized and inserted into the level, wherever it belongs.
    fn create(level: &mut Level, ctx: &mut Context) -> GameResult
    where
        Self: Sized;

    fn update(&mut self, level: &mut Level, ctx: &mut Context) -> GameResult;

    fn draw(&mut self, level: &Level, ctx: &mut Context, canvas: &mut Canvas) -> GameResult;

    fn get_hitbox(&self) -> Option<(HitboxType, Vec2)> {
        None
    }
}

#[derive(Debug, Reflect)]
pub struct EnemyContainer {
    pub missing_enemy: Option<()>,
    pub basic_enemy: Option<BasicEnemy>,
}

impl EnemyContainer {
    pub fn new() -> Self {
        Self {
            missing_enemy: None,
            basic_enemy: None,
        }
    }

    pub fn init(level: &mut Level, level_data: &LevelData, ctx: &mut Context) {
        BasicEnemy::create(level, ctx).unwrap();
    }

    pub fn enemy_ids(&self) -> &[&'static str] {
        self.reflect_type_info().as_struct().unwrap().field_names()
    }

    pub(crate) fn update(level: &mut Level, ctx: &mut Context) -> GameResult {
        level.enemies.basic_enemy =
            Self::update_enemy((&mut level.enemies).basic_enemy.take(), level, ctx);

        Ok(())
    }

    pub(crate) fn draw(level: &mut Level, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        level.enemies.basic_enemy =
            Self::draw_enemy(level.enemies.basic_enemy.take(), level, ctx, canvas);
        Ok(())
    }

    fn update_enemy<T: Enemy>(
        mut enemy: Option<T>,
        level: &mut Level,
        ctx: &mut Context,
    ) -> Option<T> {
        if let Some(some) = &mut enemy {
            some.update(level, ctx);
        }
        enemy
    }

    fn draw_enemy<T: Enemy>(
        mut enemy: Option<T>,
        level: &mut Level,
        ctx: &mut Context,
        canvas: &mut Canvas,
    ) -> Option<T> {
        if let Some(some) = &mut enemy {
            some.draw(level, ctx, canvas);
        }
        enemy
    }
}
