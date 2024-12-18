use crate::item::ItemType;
use crate::level::{Level, ProtagData};
use crate::sword::Sword;
use crate::Direction;
use ggez::graphics::{Canvas, Color, DrawParam, Drawable, Quad, Transform};
use ggez::input::keyboard::{KeyCode, KeyboardContext};
use ggez::Context;
use glam::Vec2;

pub struct Protag {
    pub position: glam::Vec2,
    pub scale: glam::Vec2,
    pub direction: Direction,
    pub current_item: ItemType,
    pub inventory: Inventory,
    pub can_move: bool,
    pub can_turn: bool,
}

impl Protag {
    pub fn new(init: &ProtagData, ctx: &mut Context) -> Self {
        Self {
            position: init.start_pos,
            current_item: ItemType::Sword,
            direction: Direction::Down,
            inventory: Inventory::new(),
            can_move: true,
            can_turn: true,
            scale: [80.0, 80.0].into(),
        }
    }

    pub fn update(level: &mut Level, ctx: &mut Context) {
        match level.protag.current_item {
            ItemType::None => (),
            ItemType::Sword => Sword::update(level, ctx),
            ItemType::Boomerang => todo!(),
            ItemType::Bow => todo!(),
            ItemType::Bomb => todo!(),
        }

        ProtagController::update(&mut level.protag, ctx);
    }

    pub fn draw(level: &mut Level, canvas: &mut Canvas) {
        canvas.draw(
            &Quad,
            DrawParam::new()
                .dest(level.protag.position)
                .color(Color::RED)
                .scale(level.protag.scale),
        );

        match level.protag.current_item {
            ItemType::None => (),
            ItemType::Sword => Sword::draw(level, canvas),
            ItemType::Boomerang => todo!(),
            ItemType::Bow => todo!(),
            ItemType::Bomb => todo!(),
        }
    }
}

impl Entity for Protag {
    fn access(level: &mut Level) -> &mut Self
    where
        Self: Sized,
    {
        todo!()
    }
}

impl GlobalUpdate for Protag {
    fn update(level: &mut Level, ctx: &mut Context)
    where
        Self: Sized,
    {
        todo!()
    }
}

pub struct ProtagController;

static PLAYER_SPEED: f32 = 6.0;

impl ProtagController {
    pub fn update(protag: &mut Protag, ctx: &mut ggez::Context) {
        let input = get_input_axis(&ctx.keyboard);

        if protag.can_turn {
            protag.direction = get_direction(input, protag.direction)
        }

        if protag.can_move {
            protag.position += input.normalize_or_zero() * PLAYER_SPEED
        }
    }
}

fn get_direction(input: Vec2, default: Direction) -> Direction {
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

fn get_input_axis(keyboard_context: &KeyboardContext) -> Vec2 {
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

pub struct Inventory {
    pub sword: Sword,
}

impl Inventory {
    fn new() -> Self {
        Self {
            sword: Sword::new(),
        }
    }
}
