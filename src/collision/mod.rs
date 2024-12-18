use crate::Direction;
use ggez::graphics::{
    Canvas, Color, DrawParam, GraphicsContext, Mesh, MeshData, Quad, Rect, Vertex,
};
use ggez::GameResult;
use glam::Vec2;

pub enum HitboxType<'a, 'string, 'frame, 'hitbox> {
    /// A singular hitbox.
    Singular(&'a Hitbox),
    /// A compound of hitboxes, consisting of at least one (but typically more) hitboxes.
    Compound(&'a HitboxFrame<'hitbox>),
    /// A list of compound hitboxes, to be iterated upon frame by frame.
    ///
    /// The second item (usize) is the current frame that should be active.
    String(&'a HitboxFrameString<'string, 'frame, 'hitbox>, usize),
}

// Hitboxes can be rotated around an origin
#[derive(Debug, Clone)]
pub struct Hitbox {
    rect: Rect,
    direction: Direction,
}

impl Hitbox {
    pub const fn new(rect: Rect) -> Self {
        Self {
            rect,
            direction: Direction::Right,
        }
    }

    /// Creates a new hitbox, centered at (0,0) + point, with the width and height both equalling the size divided by two.
    ///
    /// Exists as a helper function to make hitbox creation more intuitive
    pub const fn point_size(point: Vec2, size: f32) -> Self {
        Self {
            rect: Rect {
                x: point.x - (size / 2.0),
                y: point.y - (size / 2.0),
                w: size,
                h: size,
            },
            direction: Direction::Right,
        }
    }

    /// Rotates the hitbox so that it points in the given direction.
    ///
    /// Does nothing if the hitbox is already pointing in that direction.
    pub fn as_direction(mut self, direction: Direction) -> Self {
        self.rect
            .rotate(direction.to_angle() - self.direction.to_angle());
        self
    }

    pub fn colliding(&self, other: &HitboxType, offset: Vec2, other_offset: Vec2) -> bool {
        match other {
            HitboxType::Singular(single) => self.colliding_single(&single, offset, other_offset),
            HitboxType::Compound(compound) => self.colliding_frame(&compound, offset, other_offset),
            HitboxType::String(_, _) => todo!(),
        }
    }

    pub fn colliding_single(&self, other: &Hitbox, offset: Vec2, other_offset: Vec2) -> bool {
        Rect::from([
            self.rect.x + offset.x,
            self.rect.y + offset.y,
            self.rect.w,
            self.rect.h,
        ])
        .overlaps(&Rect::from([
            other.rect.x + other_offset.x,
            other.rect.y + other_offset.y,
            other.rect.w,
            other.rect.h,
        ]))
    }

    pub fn colliding_frame(&self, other: &HitboxFrame, offset: Vec2, other_offset: Vec2) -> bool {
        let does_this_self_hitbox_overlap_with_other =
            |a: bool, other: &Hitbox| a || self.colliding_single(other, offset, other_offset);

        other
            .0
            .into_iter()
            .fold(false, does_this_self_hitbox_overlap_with_other)
    }

    pub fn draw(
        &self,
        gfx: &mut GraphicsContext,
        canvas: &mut Canvas,
        world_offset: Vec2,
        color: Option<Color>,
    ) -> GameResult {
        let vertices: [[f32; 2]; 5] = [
            [self.rect.left(), self.rect.top()],
            [self.rect.right(), self.rect.top()],
            [self.rect.right(), self.rect.bottom()],
            [self.rect.left(), self.rect.bottom()],
            [self.rect.left(), self.rect.top()],
        ];

        let vertices: Vec<Vec2> = vertices
            .into_iter()
            .map(|value| Vec2::from(value) + world_offset)
            .collect();

        canvas.draw(
            &Mesh::new_line(gfx, &vertices, 4.0, Color::WHITE)?,
            DrawParam::new().color(color.unwrap_or(Color::WHITE)).z(999),
        );
        Ok(())
    }
}

pub static TEST_HITBOX_STRING: HitboxFrameString = HitboxFrameString::new(&[
    &HitboxFrame::new(&[
        Hitbox::point_size(Vec2::ZERO, 5.0),
        Hitbox::point_size(Vec2::new(25.0, 25.0), 10.0),
    ]),
    &HitboxFrame::new(&[
        Hitbox::point_size(Vec2::new(50.0, 25.0), 15.0),
        Hitbox::point_size(Vec2::new(75.0, 25.0), 20.0),
    ]),
    &HitboxFrame::new(&[
        Hitbox::point_size(Vec2::new(50.0, 25.0), 15.0),
        Hitbox::point_size(Vec2::new(75.0, 25.0), 20.0),
    ]),
    &HitboxFrame::new(&[
        Hitbox::point_size(Vec2::new(50.0, 25.0), 15.0),
        Hitbox::point_size(Vec2::new(75.0, 25.0), 20.0),
    ]),
]);

/// A set of [`Hitbox`]es, made to operate together as one more complex hitbox.
///
/// The second item ([`Rect`]) is the bounding box, cached with the frame to avoid recalculating it every collision.
///
/// See [`Hitbox`] for details on each individual hitbox.
///
/// Also see [`HitboxFrameString`] for a set of [`HitboxFrame`]'s that can be interchanged frame-by-frame.
pub struct HitboxFrame<'hitbox>(pub &'hitbox [Hitbox], Rect);

#[inline]
const fn min(a: f32, b: f32) -> f32 {
    match a > b {
        true => b,
        false => a,
    }
}

#[inline]
const fn max(a: f32, b: f32) -> f32 {
    match a > b {
        true => a,
        false => b,
    }
}

impl<'hitbox> HitboxFrame<'hitbox> {
    pub const fn new(hitboxes: &'hitbox [Hitbox]) -> Self {
        Self(hitboxes, HitboxFrame::calculate_bounding_box(hitboxes))
    }

    pub const fn calculate_bounding_box(hitboxes: &'hitbox [Hitbox]) -> Rect {
        let mut bounding_box: Rect = Rect::zero();
        let mut i = 0;
        while i < hitboxes.len() {
            bounding_box = {
                let other = hitboxes[i].rect;
                let x = min(bounding_box.x, other.x);
                let y = min(bounding_box.y, other.y);
                let w = max(
                    (&bounding_box).x + (&bounding_box).w,
                    (&other).x + (&other).w,
                ) - x;
                let h = max(
                    (&bounding_box).y + (&bounding_box).h,
                    (&other).y + (&other).h,
                ) - y;
                Rect { x, y, w, h }
            };
            i += 1;
        }
        bounding_box
    }

    pub fn colliding(&self, other: &HitboxType, offset: Vec2, other_offset: Vec2) -> bool {
        match other {
            HitboxType::Singular(single) => self.colliding_single(&single, offset, other_offset),
            HitboxType::Compound(compound) => self.colliding_frame(compound, offset, other_offset),
            HitboxType::String(string, index) => {
                let compound = string
                    .0
                    .get(*index)
                    .expect("expected a valid index that correlates to a frame");
                self.colliding_frame(compound, offset, other_offset)
            }
        }
    }

    pub fn colliding_single(&self, other: &Hitbox, offset: Vec2, other_offset: Vec2) -> bool {
        for rect in self.0.iter() {
            if rect.colliding_single(&other, offset, other_offset) {
                return true;
            }
        }
        false
    }

    pub fn colliding_frame(
        &self,
        compound: &&HitboxFrame,
        offset: Vec2,
        other_offset: Vec2,
    ) -> bool {
        // wouldve named offset_bounding_box, but this more turse way is much nicer with formatting
        let offset_box = |b: Rect, o: Vec2| Rect::new(b.x + o.x, b.y + o.y, b.w, b.h);
        let self_bounding_box = offset_box(self.1.clone(), offset);
        let other_bounding_box = offset_box(compound.1.clone(), other_offset);

        if !self_bounding_box.overlaps(&other_bounding_box) {
            return false;
        }

        self.0
            .into_iter()
            .filter(|hitbox: &&Hitbox| hitbox.rect.overlaps(&other_bounding_box)) // optimization filter, check for bounding box to save compound checks
            .filter(|hitbox: &&Hitbox| hitbox.colliding_frame(&compound, offset, other_offset)) // returns any hitboxes that are overlapping with the other compound hitbox
            .next() // if any overlaps exist, then some
            .is_some()
    }

    pub fn draw(
        &self,
        gfx: &mut GraphicsContext,
        canvas: &mut Canvas,
        offset: Vec2,
        color: Color,
    ) -> GameResult {
        for hitbox in self.0.iter() {
            hitbox.draw(gfx, canvas, offset, Some(color))?;
        }
        Ok(())
    }
}

impl HitboxType<'_, '_, '_, '_> {
    pub fn is_colliding(&self, other: &HitboxType, offset: Vec2, other_offset: Vec2) -> bool {
        match self {
            HitboxType::Singular(singular) => singular.colliding(other, offset, other_offset),
            HitboxType::Compound(compound) => compound.colliding(other, offset, other_offset),
            HitboxType::String(string, index) => {
                string
                    .0
                    .get(*index)
                    .unwrap()
                    .colliding(other, offset, other_offset)
            }
        }
    }
}

pub type StaticHitboxFrameString = HitboxFrameString<'static, 'static, 'static>;

/// A set of hitbox sets, to be iterated through frame by frame.
///
/// An example of this would be a fighting game attack, with the "frame data".
/// Each frame has a set of hitboxes, and
pub struct HitboxFrameString<'string, 'frame, 'hitbox>(pub &'string [&'frame HitboxFrame<'hitbox>]);

impl<'string, 'frame, 'hitbox> HitboxFrameString<'string, 'frame, 'hitbox> {
    pub const fn new(zero: &'string [&'frame HitboxFrame<'hitbox>]) -> Self {
        Self(zero)
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub fn colliding(
        &self,
        frame: usize,
        other: &HitboxType,
        offset: Vec2,
        other_offset: Vec2,
    ) -> bool {
        HitboxType::String(&self, frame).is_colliding(other, offset, other_offset)
    }

    pub fn draw(
        &self,
        gfx: &mut GraphicsContext,
        canvas: &mut Canvas,
        index: usize,
        offset: Vec2,
        color: Color,
    ) -> GameResult {
        if let Some(frame) = self.0.get(index) {
            frame.draw(gfx, canvas, offset, color)?;
        }
        Ok(())
    }
}
