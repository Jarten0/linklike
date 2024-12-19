use super::Protag;
use crate::Direction;
use ggez::input::keyboard::{KeyCode, KeyboardContext};
use glam::Vec2;

pub struct ProtagController {
    pub can_move: bool,
    pub can_turn: bool,
}

pub(crate) static PLAYER_SPEED: f32 = 6.0;

impl ProtagController {
    pub fn update(protag: &mut Protag, ctx: &mut ggez::Context) {
        let input = get_input_axis(&ctx.keyboard);

        if protag.controller.can_turn {
            protag.direction = get_direction(input, protag.direction)
        }

        if protag.controller.can_move {
            protag.position += input.normalize_or_zero() * PLAYER_SPEED
        }
    }

    pub(crate) fn new() -> Self {
        Self {
            can_move: true,
            can_turn: true,
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
