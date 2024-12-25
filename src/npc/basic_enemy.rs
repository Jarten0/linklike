use super::{DamageTransfer, Enemy, EnemyStats, REMOVE_ENEMY_WORKAROUND};
use crate::collision::{Hitbox, HitboxType};
use crate::get::Get;
use crate::level::Level;
use crate::Direction;
use bevy_reflect::{GetField, Reflect};
use ggez::graphics::{Canvas, Color, Rect};
use ggez::{Context, GameError, GameResult};
use glam::Vec2;

#[derive(Debug, Reflect)]
pub struct BasicEnemy {
    #[reflect(ignore)]
    pub position: glam::Vec2,
    pub hurtbox: Hitbox,
    pub stats: EnemyStats,
    pub speed: f32,
}

pub struct OverheadAttack {
    pub hitboxes: [Vec<Rect>],
}

impl Enemy for BasicEnemy {
    fn create(level: &mut Level, ctx: &mut Context) -> GameResult
    where
        Self: Sized,
    {
        let mut filtered = [
            &mut level.enemies.basic_enemy,
            &mut level.enemies.basic_enemy2,
            &mut level.enemies.basic_enemy3,
        ]
        .into_iter()
        .filter(|value| value.is_none());
        let next = filtered.next().unwrap();

        let count = filtered.count() as f32;
        match next.replace(BasicEnemy {
            position: Vec2::new((100.0 * count) + 100.0, 50.0),
            hurtbox: Hitbox::point_size(Vec2::ZERO, 30.0),
            stats: EnemyStats {
                health: 20,
                damage: 7,
                iframes: 0,
            },
            speed: 1.0 + (count / 3.0),
        }) {
            Some(some) => Err(GameError::CustomError("Enemy already exists".to_string())),
            None => Ok(()),
        }
    }

    fn update(&mut self, level: &mut Level, ctx: &mut Context) -> GameResult {
        if self.stats.health <= 0 {
            return GameResult::Err(GameError::CustomError(REMOVE_ENEMY_WORKAROUND.to_string()));
        }
        let distance = level.protag.position - self.position;
        self.position += distance.normalize() * self.speed * (distance.length() / 48.0);
        if self.stats.iframes > 0 {
            self.stats.iframes -= 1;
        }
        Ok(())
    }

    fn draw(&mut self, level: &Level, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        self.hurtbox.draw(&mut ctx.gfx, canvas, self.position, {
            if self.stats.health > 0 {
                if self.stats.iframes > 0 {
                    Some(Color::CYAN)
                } else {
                    Some(Color {
                        r: 0.0,
                        g: 0.5,
                        b: 0.5,
                        a: 1.0,
                    })
                }
            } else {
                Some(Color::WHITE)
            }
        })
    }

    fn get_hitbox(&self) -> Option<(crate::collision::HitboxType, Vec2)> {
        Some((HitboxType::Singular(&self.hurtbox), self.position))
    }

    fn on_hit(&mut self, stats: DamageTransfer) -> bool {
        if self.stats.iframes == 0 {
            self.stats.health -= stats.damage as i32;
            self.stats.iframes = 30;
            true
        } else {
            false
        }
    }
}
