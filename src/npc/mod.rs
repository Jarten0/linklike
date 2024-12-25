use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use crate::collision::HitboxType;
use crate::level::Level;
use advanced_enemy::AdvancedEnemy;
use basic_enemy::BasicEnemy;
use bevy_reflect::erased_serde::Serialize;
use bevy_reflect::prelude::ReflectDefault;
use bevy_reflect::{
    DynamicTyped, FromReflect, GetField, PartialReflect, Reflect, Reflectable, TypeData,
};
use ggez::graphics::Canvas;
use ggez::{Context, GameError, GameResult};
use glam::Vec2;

/// If this error message is returned in [`Enemy::update`], then the enemy will be removed
pub const REMOVE_ENEMY_WORKAROUND: &str = "Workaround for remove enemy";

pub mod advanced_enemy;
pub mod basic_enemy;

/// Information sent to the player or enemy when they have taken a hit, and need to calculate their new health
#[derive(Debug, Reflect, Clone)]
pub struct DamageTransfer {
    pub damage: f32,
    /// for knockback
    pub weight: f32,
}

#[derive(Debug, Reflect, Clone)]
pub struct EnemyStats {
    pub health: i32,
    pub damage: i32,
    pub iframes: usize,
}

pub trait Enemy: Reflect {
    /// The enemy will be initialized and inserted into the level, wherever it belongs.
    fn create(level: &mut Level, ctx: &mut Context) -> GameResult
    where
        Self: Sized;

    fn update(&mut self, level: &mut Level, ctx: &mut Context) -> GameResult;

    fn draw(&mut self, level: &Level, ctx: &mut Context, canvas: &mut Canvas) -> GameResult;

    fn get_hitbox(&self) -> Option<(HitboxType, Vec2)> {
        None
    }

    fn on_hit(&mut self, stats: DamageTransfer) -> bool {
        false
    }
}

#[derive(Debug, Reflect)]
pub struct EnemyContainer {
    #[reflect(ignore)]
    pub missing_enemy: Option<()>,
    pub basic_enemy: Option<BasicEnemy>,
    pub basic_enemy2: Option<BasicEnemy>,
    pub basic_enemy3: Option<BasicEnemy>,
    pub advanced_enemy: Option<AdvancedEnemy>,
}

impl EnemyContainer {
    pub fn new() -> Self {
        Self {
            missing_enemy: None,
            basic_enemy: None,
            basic_enemy2: None,
            basic_enemy3: None,
            advanced_enemy: None,
        }
    }

    pub fn init(level: &mut Level, ctx: &mut Context) {
        BasicEnemy::create(level, ctx).unwrap();
        BasicEnemy::create(level, ctx).unwrap();
        BasicEnemy::create(level, ctx).unwrap();
        AdvancedEnemy::create(level, ctx).unwrap();
    }

    pub fn enemy_ids(&self) -> &[&'static str] {
        self.reflect_type_info().as_struct().unwrap().field_names()
    }

    pub(crate) fn update(level: &mut Level, ctx: &mut Context) -> GameResult {
        level.enemies.basic_enemy =
            Self::update_enemy((&mut level.enemies).basic_enemy.take(), level, ctx).1;
        level.enemies.basic_enemy2 =
            Self::update_enemy((&mut level.enemies).basic_enemy2.take(), level, ctx).1;
        level.enemies.basic_enemy3 =
            Self::update_enemy((&mut level.enemies).basic_enemy3.take(), level, ctx).1;
        level.enemies.advanced_enemy =
            Self::update_enemy((&mut level.enemies).advanced_enemy.take(), level, ctx).1;

        Ok(())
    }

    pub(crate) fn draw(level: &mut Level, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        level.enemies.basic_enemy =
            Self::draw_enemy(level.enemies.basic_enemy.take(), level, ctx, canvas);
        level.enemies.basic_enemy2 =
            Self::draw_enemy(level.enemies.basic_enemy2.take(), level, ctx, canvas);
        level.enemies.basic_enemy3 =
            Self::draw_enemy(level.enemies.basic_enemy3.take(), level, ctx, canvas);
        level.enemies.advanced_enemy =
            Self::draw_enemy(level.enemies.advanced_enemy.take(), level, ctx, canvas);
        Ok(())
    }

    fn update_enemy<T: Enemy>(
        mut enemy: Option<T>,
        level: &mut Level,
        ctx: &mut Context,
    ) -> (GameResult, Option<T>) {
        (
            if let Some(some) = &mut enemy {
                let update = some.update(level, ctx);

                if let Err(ref err) = update {
                    if let GameError::CustomError(err) = err {
                        if err == REMOVE_ENEMY_WORKAROUND {
                            return (Ok(()), None);
                        }
                    }
                }

                update
            } else {
                Ok(())
            },
            enemy,
        )
    }

    fn draw_enemy<T: Enemy>(
        mut enemy: Option<T>,
        level: &mut Level,
        ctx: &mut Context,
        canvas: &mut Canvas,
    ) -> Option<T> {
        if let Some(some) = &mut enemy {
            some.draw(level, ctx, canvas).unwrap();
        }
        enemy
    }
}
