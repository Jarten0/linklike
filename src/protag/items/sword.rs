use std::collections::HashMap;

use crate::collision::{Hitbox, HitboxFrame, HitboxFrameString, StaticHitboxFrameString};
use crate::enemies::basic_enemy::BasicEnemy;
use crate::enemies::Enemy;
use crate::level::Level;
use crate::Direction;
use bevy_reflect::{GetField, Reflect};
use ggez::graphics::DrawParam;
use ggez::graphics::Quad;
use ggez::graphics::Rect;
use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::KeyCode;
use ggez::Context;
use glam::Vec2;

use super::ProtagItem;

#[derive(Debug, Clone, Reflect, PartialEq)]
pub struct Sword {
    pub state: SwordState,
    #[reflect(ignore)]
    pub swing: [StaticHitboxFrameString; 4],
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

pub static SWORD_SWING: HitboxFrameString = HitboxFrameString::new(&[
    &HitboxFrame::new(&[Hitbox::point_size(Vec2::new(0.0, 80.0), 40.0)]),
    &HitboxFrame::new(&[Hitbox::point_size(Vec2::new(60.0, 40.0), 40.0)]),
    &HitboxFrame::new(&[Hitbox::point_size(Vec2::new(80.0, 0.0), 40.0)]),
    &HitboxFrame::new(&[Hitbox::point_size(Vec2::new(60.0, -40.0), 40.0)]),
    &HitboxFrame::new(&[Hitbox::point_size(Vec2::new(0.0, -80.0), 40.0)]),
]);

impl Sword {
    pub(crate) const fn new() -> Self {
        Self {
            state: SwordState::Inactive,
            swing: todo!(),
        }
    }
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
            SwordState::Active {
                direction: _,
                frame,
            } => {
                *frame += 1;
                if *frame >= sword.swing.len() {
                    sword.state = SwordState::Inactive;
                    return;
                }
            }
        }
    }

    pub fn draw(level: &Level, ctx: &mut Context, canvas: &mut Canvas) {
        let sword = &level.protag.inventory.sword;
        match &sword.state {
            SwordState::Inactive => {}
            SwordState::Active { direction, frame } => {
                // let keyframe = &sword.keyframes[*frame];
                // let direction: Vec2 = direction.into();
                // canvas.draw(
                //     &Quad,
                //     DrawParam::new()
                //         .offset((Vec2::from(keyframe.point()) + Vec2::new(40., 40.)) / 80.)
                //         .color(
                //             // if *hit { Color::GREEN } else
                //             { Color::WHITE },
                //         )
                //         .rotation(direction.angle_between(Vec2::Y))
                //         .scale(Vec2::splat(80.))
                //         // .dest(Vec2::splat(350.)),
                //         .dest(level.protag.position + (level.protag.scale / 2.))
                //         .z(-1),
                // );
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
                .offset(Vec2::from([0., 0.]) / Vec2::from([40., 40.]).length())
                // .color(if *hit { Color::GREEN } else { Color::WHITE })
                .rotation(-Vec2::from(level.protag.direction).angle_between(Vec2::ONE))
                .scale(Vec2::from([40., 40.]))
                .dest(level.protag.position + (level.protag.scale / 2.)),
        );
    }
}
