use super::ProtagItem;
use crate::collision::{
    Hitbox, HitboxFrameRef, HitboxFrameString, HitboxFrameStringRef, HitboxType,
};
use crate::level::Level;
use crate::npc::advanced_enemy::AdvancedEnemy;
use crate::npc::basic_enemy::BasicEnemy;
use crate::npc::{DamageTransfer, Enemy};
use crate::{Direction, Game};
use bevy_reflect::prelude::ReflectDefault;
use bevy_reflect::{GetField, Reflect};
use ggez::graphics::DrawParam;
use ggez::graphics::Quad;
use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::KeyCode;
use ggez::Context;
use glam::Vec2;

#[derive(Debug, Clone, Reflect, PartialEq)]
#[reflect(Default)]
pub struct Sword {
    pub state: SwordState,
    #[reflect(ignore)]
    pub swing: &'static [HitboxFrameString; 4],
}

impl Default for Sword {
    fn default() -> Self {
        Self {
            state: Default::default(),
            swing: &Game::static_assets().protag.inventory.sword.swing,
        }
    }
}

#[derive(Debug, Default, Reflect, Clone, PartialEq)]
pub enum SwordState {
    #[default]
    Inactive,
    Active {
        frame: usize,
        direction: Direction,
    },
}

impl ProtagItem for Sword {
    fn active(&mut self) -> bool {
        match self.state {
            SwordState::Inactive => false,
            SwordState::Active { .. } => true,
        }
    }

    fn can_move(&mut self) -> bool {
        !self.active()
    }

    fn can_turn(&mut self) -> bool {
        !self.active()
    }
}

impl Sword {
    pub fn update(level: &mut Level, ctx: &mut Context) {
        let sword = &mut level.protag.inventory.sword;
        match &mut sword.state {
            SwordState::Inactive => {
                if ctx.keyboard.is_key_just_pressed(KeyCode::Space) {
                    sword.state = SwordState::Active {
                        direction: level.protag.direction,
                        frame: 0,
                    }
                }
            }
            SwordState::Active { direction, frame } => {
                *frame += 1;
                if *frame >= sword.swing[*direction as usize].len() {
                    sword.state = SwordState::Inactive;
                    return;
                }

                if let Some(other) = &mut level.enemies.basic_enemy {
                    if sword.swing[*direction as usize].colliding(
                        *frame,
                        HitboxType::Singular(&other.hurtbox),
                        level.protag.position,
                        other.position,
                    ) {
                        other.on_hit(DamageTransfer {
                            damage: 5.,
                            weight: 1.,
                        });
                    }
                };
            }
        }
    }

    pub fn draw(level: &Level, ctx: &mut Context, canvas: &mut Canvas) {
        let sword = &level.protag.inventory.sword;
        match &sword.state {
            SwordState::Inactive => {}
            SwordState::Active { direction, frame } => {
                let color = if let Some(other) = level
                    .enemies
                    .get_field::<Option<BasicEnemy>>("basic_enemy")
                    .unwrap_or(&mut None)
                {
                    sword.swing[*direction as usize]
                        .colliding(
                            *frame,
                            other.get_hitbox().unwrap().0,
                            level.protag.position,
                            other.get_hitbox().unwrap().1,
                        )
                        .then_some(Color::RED)
                        .unwrap_or(Color::WHITE)
                } else if let Some(other) = level
                    .enemies
                    .get_field::<Option<BasicEnemy>>("basic_enemy2")
                    .unwrap_or(&mut None)
                {
                    sword.swing[*direction as usize]
                        .colliding(
                            *frame,
                            other.get_hitbox().unwrap().0,
                            level.protag.position,
                            other.get_hitbox().unwrap().1,
                        )
                        .then_some(Color::RED)
                        .unwrap_or(Color::WHITE)
                } else if let Some(other) = level
                    .enemies
                    .get_field::<Option<BasicEnemy>>("basic_enemy3")
                    .unwrap_or(&mut None)
                {
                    sword.swing[*direction as usize]
                        .colliding(
                            *frame,
                            other.get_hitbox().unwrap().0,
                            level.protag.position,
                            other.get_hitbox().unwrap().1,
                        )
                        .then_some(Color::RED)
                        .unwrap_or(Color::WHITE)
                } else if let Some(other) = level
                    .enemies
                    .get_field::<Option<AdvancedEnemy>>("advanced_enemy")
                    .unwrap_or(&mut None)
                {
                    sword.swing[*direction as usize]
                        .colliding(
                            *frame,
                            other.get_hitbox().unwrap().0,
                            level.protag.position,
                            other.get_hitbox().unwrap().1,
                        )
                        .then_some(Color::RED)
                        .unwrap_or(Color::WHITE)
                } else {
                    Color::BLACK
                };
                sword.swing[*direction as usize]
                    .draw(&mut ctx.gfx, canvas, *frame, level.protag.position, color)
                    .unwrap()
            }
        }
        canvas.draw(
            &Quad,
            DrawParam::new()
                // .offset(Vec2::from([0., 0.]) / Vec2::from([40., 40.]).length())
                // .color(if *hit { Color::GREEN } else { Color::WHITE })
                // .rotation(-Vec2::from(level.protag.direction).angle_between(Vec2::ONE))
                // .scale(Vec2::from([40., 40.]))
                .dest(
                    level.protag.position, // + (level.protag.scale / 2.)
                ),
        );
    }
}
