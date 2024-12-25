use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use crate::collision::HitboxType;
use crate::level::Level;
use basic_enemy::BasicEnemy;
use bevy_reflect::erased_serde::Serialize;
use bevy_reflect::prelude::ReflectDefault;
use bevy_reflect::{DynamicTyped, FromReflect, PartialReflect, Reflect, TypeData};
use ggez::graphics::Canvas;
use ggez::{Context, GameResult};
use glam::Vec2;

pub mod basic_enemy;
pub mod basic_stats;

/// A set of stats that can represent common values.
/// Some stats can have various interpretations, where they may be used for various things.
#[derive(Debug, Reflect, Default)]
#[reflect(Default)]
pub struct Stats(#[reflect(ignore)] HashMap<&'static str, Box<dyn Stat>>);

impl Stats {
    pub fn get(&self, name: &'static str) -> Option<&dyn Stat> {
        self.0.get(name).map(Box::deref)
    }
    pub fn get_mut(&mut self, name: &'static str) -> Option<&mut dyn Stat> {
        self.0.get_mut(name).map(Box::deref_mut)
    }
    pub fn stat<T: Stat>(&self) -> &dyn Stat {
        self.0[T::name()].as_ref()
    }
    pub fn stat_mut<T: Stat>(&mut self) -> &mut dyn Stat {
        self.0.get_mut(T::name()).unwrap().as_mut()
    }
    pub fn set<T: Stat>(&mut self, stat: Box<dyn Stat>) -> Option<Box<dyn Stat>> {
        self.0.insert(stat.stat_name(), stat)
    }
}

impl From<Vec<Box<dyn Stat>>> for Stats {
    fn from(value: Vec<Box<(dyn Stat + 'static)>>) -> Self {
        Self(
            value
                .into_iter()
                .map(|value: Box<dyn Stat>| (value.stat_name(), value))
                .collect(),
        )
    }
}

pub trait Stat: PartialReflect + Debug + Serialize {
    fn name() -> &'static str
    where
        Self: Sized;

    fn stat_name(&self) -> &'static str;

    fn get(&self) -> &dyn Reflect;

    fn get_mut(&mut self) -> &mut dyn Reflect;
}

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

    pub fn init(level: &mut Level, ctx: &mut Context) {
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
