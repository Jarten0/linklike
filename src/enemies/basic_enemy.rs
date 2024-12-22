use super::{Enemy, EnemyContainer};
use crate::collision::{Hitbox, HitboxType};
use crate::get::Get;
use crate::level::Level;
use bevy_reflect::{GetField, Reflect};
use ggez::graphics::{Canvas, Color, Rect};
use ggez::{Context, GameError, GameResult};
use glam::Vec2;

#[derive(Debug, Reflect, Default)]
pub struct BasicEnemy {
    // #[reflect(ignore)]
    // pub sprite: Quad,
    #[reflect(ignore)]
    pub position: glam::Vec2,
    pub hurtbox: Hitbox,
}

pub struct OverheadAttack {
    pub hitboxes: [Vec<Rect>],
}

// pub struct

impl Enemy for BasicEnemy {
    fn create(level: &mut Level, ctx: &mut Context) -> GameResult
    where
        Self: Sized,
    {
        match level.enemies.basic_enemy.replace(BasicEnemy {
            position: Vec2::default(),
            hurtbox: Hitbox::point_size(Vec2::ZERO, 30.0),
        }) {
            Some(some) => Err(GameError::CustomError("Enemy already exists".to_string())),
            None => Ok(()),
        }
    }

    fn update(&mut self, level: &mut Level, ctx: &mut Context) -> GameResult {
        self.position += (level.protag.position - self.position).normalize();
        Ok(())
    }

    fn draw(&mut self, level: &Level, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        self.hurtbox
            .draw(&mut ctx.gfx, canvas, self.position, Some(Color::CYAN))
    }

    fn get_hitbox(&self) -> Option<(crate::collision::HitboxType, Vec2)> {
        Some((HitboxType::Singular(&self.hurtbox), self.position))
    }
}
