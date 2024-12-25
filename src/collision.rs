use crate::Direction;
use bevy_reflect::Reflect;
use ggez::graphics::{Canvas, Color, DrawParam, GraphicsContext, Mesh, Rect};
use ggez::{mint, GameResult};
use glam::Vec2;
use lerp::Lerp;
use std::convert::AsRef;
use std::iter::Iterator;

pub enum OwnedHitboxType {
    /// A singular hitbox.
    Singular(Hitbox),
    /// A compound of hitboxes, consisting of at least one (but typically more) hitboxes.
    Compound(HitboxFrame),
    /// A list of compound hitboxes, to be iterated upon frame by frame.
    ///
    /// The second item (usize) is the current frame that should be active.
    String(HitboxFrameString, usize),
}

impl OwnedHitboxType {
    pub fn as_ref(&self) -> HitboxType {
        match self {
            OwnedHitboxType::Singular(singular) => HitboxType::Singular(singular),
            OwnedHitboxType::Compound(compound) => HitboxType::Compound(compound.borrow()),
            OwnedHitboxType::String(string, us) => HitboxType::BorrowedString(string.borrow(), *us),
        }
    }
}

pub enum HitboxType<'string, 'frame, 'hitbox> {
    /// A singular hitbox.
    Singular(&'hitbox Hitbox),
    /// A compound of hitboxes, consisting of at least one (but typically more) hitboxes.
    Compound(HitboxFrameRef<'frame>),
    /// A list of compound hitboxes, to be iterated upon frame by frame.
    ///
    /// The second item (usize) is the current frame that should be active.
    String(HitboxFrameStringRef<'string, 'frame>, usize),
    BorrowedString(BorrowedHitboxFrameString<'frame>, usize),
}

// Hitboxes can be rotated around an origin
#[derive(Debug, Default, Clone, Reflect, PartialEq)]
pub struct Hitbox {
    #[reflect(ignore)]
    rect: Rect,
}

impl Hitbox {
    pub const fn new(rect: Rect) -> Self {
        Self { rect }
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
        }
    }

    /// Rotates the hitbox so that it points in the given direction.
    ///
    /// Does nothing if the hitbox is already pointing in that direction.
    pub const fn as_direction(
        mut self,
        direction: Direction,
        mut clockwise_rotations: i32,
    ) -> Self {
        // self.rect
        //     .rotate(direction.to_angle() - self.direction.to_angle());

        while clockwise_rotations > 0 {
            self.rotate_clockwise();
            clockwise_rotations -= 1;
        }

        self
    }

    /// Rotates the hitbox so that it points in the given direction.
    ///
    /// Does nothing if the hitbox is already pointing in that direction.
    pub const fn as_direction_centered(
        mut self,
        direction: Direction,
        mut clockwise_rotations: i32,
    ) -> Self {
        // self.rect
        //     .rotate(direction.to_angle() - self.direction.to_angle());

        while clockwise_rotations > 0 {
            self.rotate_clockwise();
            clockwise_rotations -= 1;
        }

        self
    }

    #[inline]
    pub const fn rotate_clockwise(&mut self) {
        let mut points: [[f32; 2]; 4] = [
            [self.rect.x, self.rect.y],
            [self.rect.x, self.rect.y + self.rect.h],
            [self.rect.x + self.rect.w, self.rect.y],
            [self.rect.x + self.rect.w, self.rect.y + self.rect.h],
        ];

        points[0] = [points[0][1], -points[0][0]];
        points[1] = [points[1][1], -points[1][0]];
        points[2] = [points[2][1], -points[2][0]];
        points[3] = [points[3][1], -points[3][0]];

        self.rect = Self::rect_from_points(points);
    }

    #[inline]
    pub const fn rect_from_points(points: [[f32; 2]; 4]) -> Rect {
        let min_x = min(
            min(points[0][0], points[1][0]),
            min(points[2][0], points[3][0]),
        );
        let min_y = min(
            min(points[0][1], points[1][1]),
            min(points[2][1], points[3][1]),
        );
        let max_x = max(
            max(points[0][0], points[1][0]),
            max(points[2][0], points[3][0]),
        );
        let max_y = max(
            max(points[0][1], points[1][1]),
            max(points[2][1], points[3][1]),
        );
        Rect {
            x: min_x,
            y: min_y,
            w: max_x - min_x,
            h: max_y - min_y,
        }
    }

    pub fn lerp(&self, other: &Hitbox, t: f32) -> Hitbox {
        Hitbox::new(Rect {
            x: self.rect.x.lerp(other.rect.x, t),
            y: self.rect.y.lerp(other.rect.y, t),
            w: self.rect.w.lerp(other.rect.w, t),
            h: self.rect.h.lerp(other.rect.h, t),
        })
    }

    pub fn twine_lerp(&self, target: &Hitbox, t: f32, twine: f32) -> Hitbox {
        let target_x = self.rect.x.lerp(target.rect.x, t);
        let target_y = self.rect.y.lerp(target.rect.y, t);
        Hitbox::new(Rect {
            x: self.rect.x + ((target_x - self.rect.x) * twine),
            y: self.rect.y + ((target_y - self.rect.y) * twine),
            w: self.rect.w.lerp(target.rect.w, t),
            h: self.rect.h.lerp(target.rect.h, t),
        })
    }

    pub fn colliding(&self, other: HitboxType, offset: Vec2, other_offset: Vec2) -> bool {
        match other {
            HitboxType::Singular(single) => self.colliding_single(&single, offset, other_offset),
            HitboxType::Compound(compound) => self.colliding_frame(compound, offset, other_offset),
            HitboxType::String(string, index) => {
                self.colliding_frame(string.0[index], offset, other_offset)
            }
            HitboxType::BorrowedString(borrowed, index) => {
                self.colliding_frame(borrowed.0[index], offset, other_offset)
            }
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

    pub fn colliding_frame(&self, other: HitboxFrameRef, offset: Vec2, other_offset: Vec2) -> bool {
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

#[derive(Debug, Reflect, Clone, PartialEq)]
pub struct HitboxFrame(Vec<Hitbox>, Direction);

impl HitboxFrame {
    pub fn new(value: &[Hitbox], direction: Direction) -> Self {
        Self(value.to_vec(), direction)
    }

    /// Returns a list of hitboxes, rotated as the given direction from the given frame.
    ///
    /// [`HitboxFrame`] stores a reference to the hitboxes, and does not store them itself.
    /// So something else needs to take ownership of the hitboxes. Thus, this function returns
    /// an iterator which you should collect into somewhere yourself. Then, you can use
    /// [`HitboxFrame::new`] with a slice of the collected hitboxes to get the new [`HitboxFrame`]
    pub fn as_direction(&self, direction: Direction) -> Self {
        let mut clockwise_rotations = (direction as i32 - self.1 as i32) % 4;

        let collect = self
            .0
            .iter()
            .map(|value| {
                value
                    .clone()
                    .as_direction_centered(direction, clockwise_rotations)
            })
            .collect();
        Self(collect, direction)
    }

    pub fn borrow(&self) -> HitboxFrameRef {
        HitboxFrameRef::from_hitboxes(self.1, &self.0)
    }
}

#[derive(Debug, Default, Clone, Reflect, PartialEq)]
pub struct HitboxFrameString(pub Vec<HitboxFrame>);

impl HitboxFrameString {
    pub fn new(frames: Vec<HitboxFrame>) -> Self {
        Self(frames)
    }

    pub fn from_frames(zero: &[HitboxFrame]) -> Self {
        Self(
            zero.iter()
                .map(|value| value.clone())
                .collect::<Vec<HitboxFrame>>(),
        )
    }

    pub fn from_hitboxes(
        zero: impl IntoIterator<Item = impl AsRef<[Hitbox]>>,
        direction: Direction,
    ) -> Self {
        let zero: Vec<HitboxFrame> = zero
            .into_iter()
            .map(|value| HitboxFrame::new(value.as_ref(), direction))
            .collect();

        Self(zero)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_direction(
        hitbox_frame_string: HitboxFrameStringRef<'_, '_>,
        direction: Direction,
    ) -> Self
// where
    //     B: for<'a> FromIterator<I>,
    //     I: FromIterator<Hitbox> + AsRef<[Hitbox]>,
    {
        Self(
            hitbox_frame_string
                .0
                .into_iter()
                .map(|value: &HitboxFrameRef<'_>| value.to_owned().as_direction(direction))
                .collect(),
        )
    }

    pub fn colliding(
        &self,
        frame: usize,
        other: HitboxType,
        offset: Vec2,
        other_offset: Vec2,
    ) -> bool {
        let binding = self.borrow();
        HitboxType::String(binding.borrow(), frame).is_colliding(other, offset, other_offset)
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
            frame.borrow().draw(gfx, canvas, offset, color)?;
        }
        Ok(())
    }

    pub fn borrow(&self) -> BorrowedHitboxFrameString {
        BorrowedHitboxFrameString::new(self)
    }
}

/// A set of [`Hitbox`]es, made to operate together as one more complex hitbox.
///
/// The second item ([`Rect`]) is the bounding box, cached with the frame to avoid recalculating it every collision.
///
/// See [`Hitbox`] for details on each individual hitbox.
///
/// Also see [`HitboxFrameString`] for a set of [`HitboxFrame`]'s that can be interchanged frame-by-frame.
#[derive(Debug, Reflect, Clone, Copy, PartialEq)]
pub struct HitboxFrameRef<'hitbox>(pub &'hitbox [Hitbox], Direction, #[reflect(ignore)] Rect);

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

impl<'hitbox> HitboxFrameRef<'hitbox> {
    pub const fn right(hitboxes: &'hitbox [Hitbox]) -> Self {
        Self(
            hitboxes,
            Direction::Right,
            HitboxFrameRef::calculate_bounding_box(hitboxes),
        )
    }
    pub const fn up(hitboxes: &'hitbox [Hitbox]) -> Self {
        Self(
            hitboxes,
            Direction::Up,
            HitboxFrameRef::calculate_bounding_box(hitboxes),
        )
    }
    pub const fn left(hitboxes: &'hitbox [Hitbox]) -> Self {
        Self(
            hitboxes,
            Direction::Left,
            HitboxFrameRef::calculate_bounding_box(hitboxes),
        )
    }
    pub const fn down(hitboxes: &'hitbox [Hitbox]) -> Self {
        Self(
            hitboxes,
            Direction::Down,
            HitboxFrameRef::calculate_bounding_box(hitboxes),
        )
    }

    pub const fn from_hitboxes(direction: Direction, hitboxes: &'hitbox [Hitbox]) -> Self {
        Self(
            hitboxes,
            direction,
            HitboxFrameRef::calculate_bounding_box(hitboxes),
        )
    }

    /// Calculate one bounding box that encompasses all of the given hitboxes
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

    pub fn colliding(&self, other: HitboxType, offset: Vec2, other_offset: Vec2) -> bool {
        match other {
            HitboxType::Singular(single) => self.colliding_single(single, offset, other_offset),
            HitboxType::Compound(compound) => self.colliding_frame(compound, offset, other_offset),
            HitboxType::String(string, index) => {
                self.colliding_frame(string.0[index], offset, other_offset)
            }
            HitboxType::BorrowedString(borrowed, index) => {
                self.colliding_frame(borrowed.0[index], offset, other_offset)
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
        compound: HitboxFrameRef,
        offset: Vec2,
        other_offset: Vec2,
    ) -> bool {
        // wouldve named offset_bounding_box, but this more turse way is much nicer with formatting
        let offset_box = |b: Rect, o: Vec2| Rect::new(b.x + o.x, b.y + o.y, b.w, b.h);
        let self_bounding_box = offset_box(self.2.clone(), offset);
        let other_bounding_box = offset_box(compound.2.clone(), other_offset);

        if !self_bounding_box.overlaps(&other_bounding_box) {
            return false;
        }

        self.0
            .into_iter()
            .filter(|hitbox: &&Hitbox| hitbox.rect.overlaps(&other_bounding_box)) // optimization filter, check for bounding box to save compound checks
            .filter(|hitbox: &&Hitbox| hitbox.colliding_frame(compound, offset, other_offset)) // returns any hitboxes that are overlapping with the other compound hitbox
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

impl HitboxFrameRef<'_> {
    pub fn to_owned(&self) -> HitboxFrame {
        HitboxFrame::new(self.0, self.1)
    }
}

impl HitboxType<'_, '_, '_> {
    pub fn is_colliding(&self, other: HitboxType, offset: Vec2, other_offset: Vec2) -> bool {
        match self {
            HitboxType::Singular(singular) => singular.colliding(other, offset, other_offset),
            HitboxType::Compound(compound) => compound.colliding(other, offset, other_offset),
            HitboxType::String(string, index) => {
                string.0[*index].colliding(other, offset, other_offset)
            }
            HitboxType::BorrowedString(borrowed, index) => {
                borrowed.0[*index].colliding(other, offset, other_offset)
            }
        }
    }
}

/// A set of hitbox sets, to be iterated through frame by frame.
///
/// An example of this would be a fighting game attack, with the "frame data".
/// Each frame has a set of hitboxes, which detail a shape that can be used for complex attacks or collision.
/// Each string has a set of frames, which can be used to "animate" a shape that changes form across time, or by other metrics.
///
/// Since both this and [`HitboxFrame`] operate off of borrowed data, it's easy to reuse hitbox data wherever needed.
///
#[derive(Debug, Default, Reflect, Clone, Copy, PartialEq)]
pub struct HitboxFrameStringRef<'string, 'hitbox>(pub &'string [HitboxFrameRef<'hitbox>]);

pub type StaticHitboxFrameString = HitboxFrameStringRef<'static, 'static>;

impl<'string, 'hitbox> HitboxFrameStringRef<'string, 'hitbox> {
    pub fn new_with_direction(zero: &'string [HitboxFrameRef<'hitbox>]) -> Self {
        Self(zero)
    }

    pub const fn new(zero: &'string [HitboxFrameRef<'hitbox>]) -> Self {
        Self(zero)
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub fn to_direction(self, direction: Direction) -> HitboxFrameString {
        HitboxFrameString::new(
            self.0
                .iter()
                .map(|value: &HitboxFrameRef<'_>| value.to_owned().as_direction(direction))
                .collect(),
        )
    }

    pub fn colliding(
        &self,
        frame: usize,
        other: HitboxType,
        offset: Vec2,
        other_offset: Vec2,
    ) -> bool {
        HitboxType::String(*self, frame).is_colliding(other, offset, other_offset)
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

pub struct BorrowedHitboxFrameString<'a>(Vec<HitboxFrameRef<'a>>);

impl<'a> BorrowedHitboxFrameString<'a> {
    pub fn new(borrowing: &'a HitboxFrameString) -> Self {
        let collect = borrowing
            .0
            .iter()
            .map(|value| value.borrow())
            .collect::<Vec<HitboxFrameRef>>();
        Self(collect)
    }

    pub fn borrow(&self) -> HitboxFrameStringRef {
        HitboxFrameStringRef::new(&self.0)
    }
}
