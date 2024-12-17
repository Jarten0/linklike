use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::{self, KeyboardContext};
use glam::Vec2;
use level::Level;
use player::Protag;

fn main() {
    let (mut ctx, event) = ggez::ContextBuilder::new("linklike", "jarten")
        .build()
        .expect("could not build :(");

    let state = Game::new(&mut ctx);

    ggez::event::run(ctx, event, state);
}

struct Game {
    level: Level,
}

impl Game {
    fn new(ctx: &mut ggez::Context) -> Self {
        let level = Level::new(ctx);
        Self { level }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        Protag::update(&mut self.level, ctx);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = Canvas::from_frame(&ctx.gfx, Color::BLACK);

        Protag::draw(&self.level, &mut canvas);

        canvas.finish(&mut ctx.gfx)
    }
}

mod level {
    use crate::player::Protag;
    use ggez::Context;
    use glam::Vec2;

    pub struct Level {
        pub protag: Protag,
    }

    impl Level {
        pub fn new(ctx: &mut Context) -> Self {
            let init: ProtagInfo = ProtagInfo {
                start_pos: Vec2::ONE * 500.0,
            };
            Self {
                protag: Protag::new(&init, ctx),
            }
        }
    }

    pub(crate) struct ProtagInfo {
        pub start_pos: glam::Vec2,
    }
}

mod player {
    use crate::item::ItemType;
    use crate::level::{Level, ProtagInfo};
    use crate::Direction;
    use ggez::graphics::{Canvas, Color, DrawParam, Drawable, Quad, Transform};
    use ggez::input::keyboard::{KeyCode, KeyboardContext};
    use ggez::Context;
    use glam::Vec2;
    use sword::Sword;

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
        pub fn new(init: &ProtagInfo, ctx: &mut Context) -> Self {
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

        pub fn draw(level: &Level, canvas: &mut Canvas) {
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
        pub sword: sword::Sword,
    }

    impl Inventory {
        fn new() -> Self {
            Self {
                sword: sword::Sword::new(),
            }
        }
    }

    mod sword {
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
    }
}

mod item {
    #[derive(Debug, Clone, Copy, Default)]
    pub enum ItemType {
        #[default]
        None,
        Sword,
        Boomerang,
        Bow,
        Bomb,
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for glam::Vec2 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Vec2::NEG_Y,
            Direction::Down => Vec2::Y,
            Direction::Left => Vec2::NEG_X,
            Direction::Right => Vec2::X,
        }
    }
}

impl From<&Direction> for glam::Vec2 {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Up => Vec2::NEG_Y,
            Direction::Down => Vec2::Y,
            Direction::Left => Vec2::X,
            Direction::Right => Vec2::NEG_X,
        }
    }
}
