use ggez::event::EventHandler;
use glam::Vec2;

fn main() {
    let (mut ctx, event) = ggez::ContextBuilder::new("linklike", "jarten")
        .build()
        .expect("could not build :(");

    let state = Game::new(&mut ctx);

    ggez::event::run(ctx, event, state);
}

struct Game {}

impl Game {
    fn new(ctx: &ggez::Context) -> Self {
        Self {}
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        todo!()
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        todo!()
    }
}

mod level {
    use glam::Vec2;

    pub struct Level {
        pub protag: ProtagInfo,
    }

    impl Default for Level {
        fn default() -> Self {
            Self {
                protag: ProtagInfo {
                    start_pos: Vec2::ZERO,
                },
            }
        }
    }

    pub(crate) struct ProtagInfo {
        pub start_pos: glam::Vec2,
    }
}

mod player {
    use ggez::graphics::{Canvas, Color, DrawParam, Drawable, Quad, Rect, Transform};
    use ggez::Context;
    use glam::Vec2;

    use crate::item::{Item, ItemType};
    use crate::level::Level;
    use crate::Direction;

    pub struct Protag {
        pub position: glam::Vec2,
        pub direction: Direction,
        pub current_item: ItemType,
        pub inventory: Inventory,
        pub can_move: bool,
    }

    impl Protag {
        pub fn new(level: &Level, ctx: &mut Context) -> Self {
            Self {
                position: level.protag.start_pos,
                current_item: ItemType::Sword,
                direction: Direction::Down,
                inventory: Inventory::new(),
                can_move: true,
            }
        }

        pub fn update(&mut self, ctx: &mut Context, level: &mut Level) {
            self.inventory
                .get_item_mut(self.current_item)
                .update(ctx, level);
        }

        pub fn draw(&self, canvas: &mut Canvas) {
            canvas.draw(
                &Quad,
                DrawParam::new().dest(self.position).color(Color::RED),
            );
            self.inventory.get_item(self.current_item).draw(canvas);
        }
    }

    pub struct Inventory {
        sword: Sword,
    }

    impl Inventory {
        pub fn get_item(&self, id: ItemType) -> &dyn Item {
            match id {
                ItemType::Sword => &self.sword,
                ItemType::Boomerang => todo!(),
                ItemType::Bow => todo!(),
                ItemType::Bomb => todo!(),
            }
        }

        pub fn get_item_mut(&mut self, id: ItemType) -> &mut dyn Item {
            match id {
                ItemType::None => todo!(),
                ItemType::Sword => &mut self.sword,
                ItemType::Boomerang => todo!(),
                ItemType::Bow => todo!(),
                ItemType::Bomb => todo!(),
            }
        }

        fn new() -> Self {
            Self {
                sword: Sword::new(),
            }
        }
    }

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
        const fn new() -> Self {
            static KEYFRAMES: [Rect; 4] = [
                Rect::new(-20., -25., 32., 32.),
                Rect::new(0., -25., 32., 32.),
                Rect::new(20., -25., 32., 32.),
                Rect::new(25., 0., 16., 32.),
            ];
            Self {
                keyframes: &KEYFRAMES,
                state: SwordState::Inactive,
            }
        }
    }

    impl Item for Sword {
        fn item_type() -> ItemType
        where
            Self: Sized,
        {
            ItemType::Sword
        }

        fn update(&mut self, ctx: &mut Context, level: &mut Level) {}

        fn draw(&self, canvas: &mut Canvas) {
            match &self.state {
                SwordState::Inactive => (),
                SwordState::Active {
                    direction,
                    frame,
                    hit,
                } => {
                    let keyframe = &self.keyframes[*frame];
                    let direction: Vec2 = direction.into();
                    canvas.draw(
                        &Quad,
                        DrawParam::new()
                            .offset(keyframe.point())
                            .color(if *hit { Color::GREEN } else { Color::WHITE })
                            .rotation(direction.angle_between(Vec2::NEG_Y))
                            .scale(keyframe.size()),
                    );
                }
            }
        }
    }
}

mod item {
    use ggez::graphics::Canvas;
    use ggez::Context;

    use crate::level::Level;

    pub trait Item {
        fn item_type() -> ItemType
        where
            Self: Sized;

        // fn item_type(&self) -> ItemType;

        fn update(&mut self, ctx: &mut Context, level: &mut Level);

        fn draw(&self, canvas: &mut Canvas);
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum ItemType {
        #[default]
        None,
        Sword,
        Boomerang,
        Bow,
        Bomb,
    }

    pub struct EmptyItem;

    impl Item for EmptyItem {
        fn item_type() -> ItemType
        where
            Self: Sized,
        {
            ItemType::None
        }

        fn update(&mut self, ctx: &mut Context, level: &mut Level) {}

        fn draw(&self, canvas: &mut Canvas) {}
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Into<glam::Vec2> for Direction {
    fn into(self) -> glam::Vec2 {
        match self {
            Direction::Up => Vec2::NEG_Y,
            Direction::Down => Vec2::Y,
            Direction::Left => Vec2::X,
            Direction::Right => Vec2::NEG_X,
        }
    }
}

impl Into<glam::Vec2> for &Direction {
    fn into(self) -> glam::Vec2 {
        match self {
            Direction::Up => Vec2::NEG_Y,
            Direction::Down => Vec2::Y,
            Direction::Left => Vec2::X,
            Direction::Right => Vec2::NEG_X,
        }
    }
}
