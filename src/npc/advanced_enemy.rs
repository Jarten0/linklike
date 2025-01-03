use crate::collision::{
    Hitbox, HitboxFrame, HitboxFrameRef, HitboxFrameString, HitboxFrameStringRef,
    StaticHitboxFrameString,
};
use crate::Direction;
use bevy_reflect::Reflect;
use ggez::graphics::{Canvas, Color, GraphicsContext, Rect};
use ggez::GameError;
use glam::Vec2;

use super::Enemy;

#[derive(Debug)]
pub struct AdvancedEnemyData {
    wide_swing: HitboxAnimation,
}

impl AdvancedEnemyData {
    pub fn new() -> Self {
        static HITBOX_DATA: HitboxFrameStringRef = HitboxFrameStringRef::new(&[
            HitboxFrameRef::down(&[Hitbox::point_size(Vec2::new(50.0, 50.0), 40.0)]),
            HitboxFrameRef::down(&[Hitbox::point_size(Vec2::new(55.0, -20.0), 40.0)]),
            HitboxFrameRef::down(&[Hitbox::point_size(Vec2::new(-80.0, 00.0), 40.0)]),
            HitboxFrameRef::down(&[Hitbox::point_size(Vec2::new(00.0, 100.0), 40.0)]),
            HitboxFrameRef::down(&[Hitbox::point_size(Vec2::new(150.0, 75.0), 40.0)]),
            HitboxFrameRef::down(&[Hitbox::point_size(Vec2::new(165.0, -150.0), 60.0)]),
            HitboxFrameRef::down(&[Hitbox::point_size(Vec2::new(20.0, -200.0), 60.0)]),
            HitboxFrameRef::down(&[Hitbox::point_size(Vec2::new(-300.0, -100.0), 80.0)]),
            HitboxFrameRef::down(&[Hitbox::point_size(Vec2::new(-350.0, 200.0), 80.0)]),
            HitboxFrameRef::down(&[Hitbox::point_size(Vec2::new(00.0, 400.0), 80.0)]),
            HitboxFrameRef::down(&[Hitbox::point_size(Vec2::new(350.0, 300.0), 80.0)]),
            HitboxFrameRef::down(&[Hitbox::point_size(Vec2::new(200.0, -50.0), 80.0)]),
            HitboxFrameRef::down(&[Hitbox::point_size(Vec2::new(00.0, -100.0), 40.0)]),
            HitboxFrameRef::down(&[Hitbox::point_size(Vec2::new(00.0, 00.0), 00.0)]),
            HitboxFrameRef::down(&[Hitbox::point_size(Vec2::new(50.0, 50.0), 0.0)]),
        ]);
        Self {
            wide_swing: HitboxAnimation::new(
                HITBOX_DATA,
                vec![4, 5, 5, 5, 5, 5, 6, 6, 4, 5, 7, 45, 10],
            ),
        }
    }
}

#[derive(Debug, Reflect)]
pub struct AdvancedEnemy {
    hurtbox: Hitbox,
    wide_swing: HitboxAnimation,
    #[reflect(ignore)]
    position: Vec2,
}

/// Machiene that turns hitboxes into lerped hitboxes
#[derive(Debug, Reflect, Clone)]
pub struct HitboxAnimation {
    pub lerped_hitboxes: HitboxFrame,
    pub hitboxes: [HitboxFrameString; 4],
    /// The amount of frames between each keyframe.
    intervals: Vec<usize>,

    frame_of_current_interval: usize,
    current_interval: usize,
    loops: bool,
    active: bool,
    twine: f32,
    direction: Direction,
}

impl HitboxAnimation {
    pub fn new(hitboxes: HitboxFrameStringRef, intervals: Vec<usize>) -> Self {
        Self {
            lerped_hitboxes: HitboxFrame::new(&[], Direction::Down),
            hitboxes: [
                hitboxes.to_direction(Direction::Right),
                hitboxes.to_direction(Direction::Up),
                hitboxes.to_direction(Direction::Left),
                hitboxes.to_direction(Direction::Down),
            ],
            intervals,
            frame_of_current_interval: 0,
            current_interval: 0,
            loops: true,
            active: true,
            twine: 1.0,
            direction: Direction::Down,
        }
    }

    pub fn loop_animations(&mut self, value: bool) {
        self.loops = value;
    }

    pub fn set_active(&mut self, value: bool) {
        self.active = value;
    }

    pub fn reset(&mut self) {
        self.active = true;
        self.current_interval = 0;
        self.frame_of_current_interval = 0;
    }

    pub fn reset_current_interval(&mut self) {
        self.frame_of_current_interval = 0;
    }

    pub fn update_animation(&mut self) -> bool {
        if !self.active {
            return false;
        }

        let updated = if self.frame_of_current_interval == 0 {
            if self.current_interval >= self.intervals.len() - 1 {
                if self.loops {
                    self.current_interval = 0;
                } else {
                    self.active = false;
                }
                true
            } else {
                self.current_interval += 1;
                self.frame_of_current_interval = self.intervals[self.current_interval];
                false
            }
        } else {
            self.frame_of_current_interval -= 1;
            false
        };

        self.lerped_hitboxes = self.lerped_hitboxes();

        updated
    }

    pub fn current_frame(&self, direction: Direction) -> HitboxFrameRef {
        self.hitboxes[direction as usize].0[self.current_interval].borrow()
    }

    pub fn next_frame(&self, direction: Direction) -> Option<HitboxFrameRef> {
        let string = &self.hitboxes[direction as usize];

        if self.current_interval + 1 >= string.0.len() {
            return None;
        }

        Some(string.0[self.current_interval + 1].borrow())
    }

    pub fn lerped_hitboxes(&self) -> HitboxFrame {
        let current_frame = self.current_frame(self.direction);

        let t = (self.intervals[self.current_interval] - self.frame_of_current_interval) as f32
            / (self.intervals[self.current_interval] as f32 + 1.0);

        let new_frame: HitboxFrame = if let Some(next_frame) = self.next_frame(self.direction) {
            HitboxFrame::new(
                &current_frame
                    .0
                    .iter()
                    .zip(next_frame.0.iter())
                    .map(|(current, next)| current.twine_lerp(next, t, self.twine))
                    .collect::<Vec<Hitbox>>(),
                self.direction,
            )
        } else {
            current_frame.to_owned()
        };

        new_frame
    }

    pub fn draw(&self, gfx: &mut GraphicsContext, canvas: &mut Canvas, offset: Vec2) {
        self.lerped_hitboxes
            .borrow()
            .draw(gfx, canvas, offset, Color::GREEN)
            .unwrap();
    }
}

impl Enemy for AdvancedEnemy {
    fn create(level: &mut crate::level::Level, ctx: &mut ggez::Context) -> ggez::GameResult
    where
        Self: Sized,
    {
        match level.enemies.advanced_enemy.replace(Self {
            hurtbox: Hitbox::point_size(Vec2::ZERO, 50.0),
            wide_swing: level.static_assets.advanced_enemy.wide_swing.clone(),
            position: Vec2::ZERO,
        }) {
            Some(some) => Err(GameError::CustomError("Enemy already exists".to_string())),
            None => Ok(()),
        }
    }

    fn update(
        &mut self,
        level: &mut crate::level::Level,
        ctx: &mut ggez::Context,
    ) -> ggez::GameResult {
        let direction = (level.protag.position - self.position).normalize();
        self.position += direction;
        if self.wide_swing.update_animation() {
            self.wide_swing.direction = Direction::from(direction)
        }
        Ok(())
    }

    fn draw(
        &mut self,
        level: &crate::level::Level,
        ctx: &mut ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
    ) -> ggez::GameResult {
        self.wide_swing.draw(&mut ctx.gfx, canvas, self.position);
        self.hurtbox
            .draw(&mut ctx.gfx, canvas, self.position, Some(Color::RED));
        Ok(())
    }
}
