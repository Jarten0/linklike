use std::any::{Any, TypeId};

use crate::collision::TEST_HITBOX_STRING;
use crate::enemies::{basic_enemy::BasicEnemy, Enemy, EnemyContainer};
use crate::player::Protag;
use bevy_reflect::{PartialReflect, Reflect, ReflectMut, ReflectRef};
use ggez::graphics::{Canvas, Color};
use ggez::{Context, GameResult};
use glam::Vec2;

/// Returns an item with the given type from the struct,
pub trait Get<Item> {
    fn get<'a>(&'a self, _id: &'static str) -> Option<&'a Item>;
    fn get_mut<'a>(&'a mut self, _id: &'static str) -> Option<&'a mut Item>;
}

/// Get `Self` from the `Container`.
pub trait Access<Container> {
    fn access<'get>(access: &'get Container, id: &'static str) -> Option<&'get Self>;
    fn access_mut<'get>(access: &'get mut Container, id: &'static str) -> Option<&'get mut Self>;
}

impl<Container: Reflect, Item: Access<Container>> Get<Item> for Container {
    fn get<'a>(&'a self, id: &'static str) -> Option<&'a Item> {
        Item::access(self, id)
    }

    fn get_mut<'a>(&'a mut self, _id: &'static str) -> Option<&'a mut Item> {
        Item::access_mut(self, _id)
    }
}

impl<C: Reflect, I: Reflect> Access<C> for I {
    fn access<'get>(access: &'get C, id: &'static str) -> Option<&'get I> {
        match access.reflect_ref() {
            ReflectRef::Struct(s) => s
                .field(id)
                .map(<dyn PartialReflect>::try_downcast_ref)
                .unwrap_or_default(),
            _ => None,
        }
    }

    fn access_mut<'get>(access: &'get mut C, id: &'static str) -> Option<&'get mut Self> {
        match access.reflect_mut() {
            ReflectMut::Struct(s) => s
                .field_mut(id)
                .map(<dyn PartialReflect>::try_downcast_mut)
                .unwrap_or_default(),
            _ => None,
        }
    }
}

#[derive(Reflect)]
pub struct Level {
    pub protag: Protag,
    pub enemies: EnemyContainer,
    i: usize,
}

impl Level {
    pub fn new(ctx: &mut Context) -> Self {
        let data: LevelData = LevelData {
            protag: ProtagData {
                start_pos: Vec2::ONE * 500.0,
            },
            enemies: vec![(<BasicEnemy as Enemy>::create, "basic_enemy")],
        };

        let mut level = Self {
            protag: Protag::new(&data.protag, ctx),
            enemies: EnemyContainer::new(),
            i: 0,
        };

        EnemyContainer::init(&mut level, &data, ctx);

        level
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        Protag::update(self, ctx);

        EnemyContainer::update(self, ctx)?;

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        Protag::draw(self, ctx, canvas)?;

        EnemyContainer::draw(self, ctx, canvas)?;

        crate::collision::TEST_HITBOX_STRING.draw(
            &mut ctx.gfx,
            canvas,
            self.i,
            Vec2::ONE * 59.0,
            Color::MAGENTA,
        )?;

        self.i += 1;
        if self.i >= TEST_HITBOX_STRING.len() {
            self.i = 0;
        }

        Ok(())
    }
}

pub struct LevelData {
    pub protag: ProtagData,
    pub enemies: Vec<(fn(&mut Level, &mut Context) -> GameResult, &'static str)>,
}

pub struct ProtagData {
    pub start_pos: glam::Vec2,
}
