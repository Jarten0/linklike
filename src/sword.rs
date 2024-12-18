use super::Protag;
use crate::item::ItemType;
use crate::level::Level;
use crate::Direction;
use ggez::graphics::Canvas;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::graphics::Quad;
use ggez::graphics::Rect;
use ggez::input::keyboard::KeyCode;
use ggez::Context;
use glam::Vec2;

pub struct Sword {
    pub keyframes: &'static [Rect],
    pub state: SwordState,
}

pub enum SwordState {
    Inactive,
    Active {
        direction: Direction,
        frame: usize,
        hit: bool,
    },
}

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
        }
    }
}

impl Sword {
    pub fn update(level: &mut Level, ctx: &mut Context) {
        let sword = &mut level.protag.inventory.sword;
        match &mut sword.state {
            SwordState::Inactive => {
                if ctx.keyboard.is_key_just_pressed(KeyCode::Space) {
                    level.protag.can_move = false;
                    level.protag.can_turn = false;
                    sword.state = SwordState::Active {
                        direction: level.protag.direction,
                        frame: 0,
                        hit: false,
                    }
                }
            }
            SwordState::Active {
                direction,
                frame,
                hit,
            } => {
                *frame += 1;
                if *frame >= sword.keyframes.len() {
                    level.protag.can_move = true;
                    level.protag.can_turn = true;

                    sword.state = SwordState::Inactive;
                    return;
                }
            }
        }
    }

    pub fn draw(level: &Level, canvas: &mut Canvas) {
        let sword = &level.protag.inventory.sword;
        match &sword.state {
            SwordState::Inactive => {}
            SwordState::Active {
                direction,
                frame,
                hit,
            } => {
                let keyframe = &sword.keyframes[*frame];
                let direction: Vec2 = direction.into();
                canvas.draw(
                    &Quad,
                    DrawParam::new()
                        .offset((Vec2::from(keyframe.point()) + Vec2::new(40., 40.)) / 80.)
                        .color(if *hit { Color::GREEN } else { Color::WHITE })
                        .rotation(direction.angle_between(Vec2::Y))
                        .scale(Vec2::splat(80.))
                        // .dest(Vec2::splat(350.)),
                        .dest(level.protag.position + (level.protag.scale / 2.))
                        .z(-1),
                );
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
