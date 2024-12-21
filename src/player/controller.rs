use super::Protag;
use crate::collision::HitboxType;
use crate::enemies::basic_enemy::BasicEnemy;
use crate::level::{Access, Level};
use crate::{level, Direction};
use bevy_reflect::Reflect;
use ggez::input::keyboard::{KeyCode, KeyboardContext};
use glam::Vec2;

#[derive(Debug, Reflect)]
pub struct ProtagController {
    pub can_move: bool,
    pub can_turn: bool,
    pub hurt: bool,
}

pub(crate) static PLAYER_SPEED: f32 = 6.0;

impl ProtagController {
    pub fn update(level: &mut Level, ctx: &mut ggez::Context) {
        let input = get_input_axis(&ctx.keyboard);

        if level.protag.controller.can_turn {
            level.protag.direction = get_direction(input, level.protag.direction)
        }

        if level.protag.controller.can_move {
            level.protag.position += input.normalize_or_zero() * PLAYER_SPEED
        }

        let basic_enemy = BasicEnemy::access_mut(&mut level.enemies, "basic_enemy").unwrap();
        if level.protag.hurtbox.colliding(
            &HitboxType::Singular(&basic_enemy.hurtbox),
            level.protag.position,
            basic_enemy.position,
        ) {
            level.protag.controller.hurt = true;
        } else {
            level.protag.controller.hurt = false;
        };
    }

    pub(crate) fn new() -> Self {
        Self {
            can_move: true,
            can_turn: true,
            hurt: false,
        }
    }
}

pub(crate) fn get_direction(input: Vec2, default: Direction) -> Direction {
    if input.x > 0.0 {
        Direction::Right
    } else if input.x < 0.0 {
        Direction::Left
    } else if input.y > 0.0 {
        Direction::Down
    } else if input.y < 0.0 {
        Direction::Up
    } else {
        default
    }
}

pub(crate) fn get_input_axis(keyboard_context: &KeyboardContext) -> Vec2 {
    let mut input = Vec2::ZERO;
    if keyboard_context.is_key_pressed(KeyCode::A) {
        input += Vec2::from(Direction::Left)
    }
    if keyboard_context.is_key_pressed(KeyCode::D) {
        input += Vec2::from(Direction::Right)
    }
    if keyboard_context.is_key_pressed(KeyCode::W) {
        input += Vec2::from(Direction::Up)
    }
    if keyboard_context.is_key_pressed(KeyCode::S) {
        input += Vec2::from(Direction::Down)
    }
    input
}
