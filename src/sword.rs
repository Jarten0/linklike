use crate::collision::{Hitbox, HitboxFrame, HitboxFrameString, StaticHitboxFrameString};
use crate::enemies::Enemy;
use crate::level::Level;
use crate::Direction;
use bevy_reflect::Reflect;
use ggez::graphics::DrawParam;
use ggez::graphics::Quad;
use ggez::graphics::Rect;
use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::KeyCode;
use ggez::Context;
use glam::Vec2;

#[derive(Debug, Clone, PartialEq)]
pub struct Sword {
    pub state: SwordState,
    // #[reflect(ignore)]
    pub keyframes: &'static [Rect],
    // #[reflect(ignore)]
    pub swing: &'static StaticHitboxFrameString,
}

impl Default for Sword {
    fn default() -> Self {
        Self {
            keyframes: Default::default(),
            state: Default::default(),
            swing: &SWORD_SWING,
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

pub static SWORD_SWING: HitboxFrameString = HitboxFrameString::new(&[
    &HitboxFrame::new(&[Hitbox::point_size(Vec2::new(0.0, 80.0), 40.0)]),
    &HitboxFrame::new(&[Hitbox::point_size(Vec2::new(60.0, 40.0), 40.0)]),
    &HitboxFrame::new(&[Hitbox::point_size(Vec2::new(80.0, 0.0), 40.0)]),
    &HitboxFrame::new(&[Hitbox::point_size(Vec2::new(60.0, -40.0), 40.0)]),
    &HitboxFrame::new(&[Hitbox::point_size(Vec2::new(0.0, -80.0), 40.0)]),
]);
impl Sword {
    pub(crate) const fn new() -> Self {
        pub(crate) static KEYFRAMES: [Rect; 8] = [
            Rect::new(-80., -40., 80., 80.),
            Rect::new(-40., -80., 80., 80.),
            Rect::new(0., -100., 80., 80.),
            Rect::new(20., -100., 80., 80.),
            Rect::new(20., -80., 80., 80.),
            Rect::new(20., -40., 80., 80.),
            Rect::new(20., 0., 80., 80.),
            Rect::new(20., 0., 80., 80.),
        ];
        Self {
            keyframes: &KEYFRAMES,
            state: SwordState::Inactive,
            swing: &SWORD_SWING,
        }
    }
}

impl Sword {
    pub fn update(level: &mut Level, ctx: &mut Context) {
        let sword = &mut level.protag.inventory.sword;
        match &mut sword.state {
            SwordState::Inactive => {
                if ctx.keyboard.is_key_just_pressed(KeyCode::Space) {
                    level.protag.controller.can_move = false;
                    level.protag.controller.can_turn = false;
                    sword.state = SwordState::Active {
                        direction: level.protag.direction,
                        frame: 0,
                    }
                }
            }
            SwordState::Active { direction, frame } => {
                *frame += 1;
                if *frame >= sword.keyframes.len() {
                    level.protag.controller.can_move = true;
                    level.protag.controller.can_turn = true;

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
                // let color = if let Some(other) = level.enemies.container.first().unwrap_or(&None) {
                //     let other = crate::enemies::basic_enemy::BasicEnemy::downcast(other);
                //      sword.swing.colliding(
                //         frame,
                //         other,
                //         level.protag.position,
                //         level.enemies.container.first().unwrap().position,
                //     ).then_some(Color::RED).unwrap_or(Color::WHITE)
                // } else {
                //     Color::BLACK
                // }
                // sword
                //     .swing
                //     .draw(&mut ctx.gfx, canvas, frame, level.protag.position, color)
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
