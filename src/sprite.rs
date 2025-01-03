use bevy_reflect::prelude::ReflectDefault;
use bevy_reflect::Reflect;
use ggez::graphics::{Canvas, DrawParam, GraphicsContext, Image, Rect, Transform};
use ggez::GameResult;
use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Reflect, Default, Serialize, Deserialize, PartialEq)]
pub struct SpriteData {
    pub texture_path: String,
    #[reflect(ignore)]
    pub src: Rect,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    #[reflect(ignore)]
    pub draw_param: DrawParam,
    #[reflect(ignore)]
    pub offset: Vec2,
}

#[derive(Debug, Reflect)]
pub struct Sprite {
    #[reflect(ignore)]
    pub texture: Option<Image>,
    #[reflect(ignore)]
    pub offset: Vec2,
    pub data: SpriteData,
}

impl Sprite {
    pub fn new(data: &'static SpriteData, gfx: &GraphicsContext) -> GameResult<Self> {
        Ok(Self {
            texture: Some(Image::from_path(gfx, data.texture_path.clone())?),
            offset: data.offset,
            data: data.to_owned(),
        })
    }

    pub fn draw(
        &self,
        canvas: &mut Canvas,
        sprite_world_offset: Vec2,
        draw_param: Option<DrawParam>,
    ) {
        if let Some(image) = self.texture.as_ref() {
            let param = draw_param.unwrap_or(self.data.draw_param);
            let dest = if let Transform::Values {
                dest,
                rotation: _,
                scale: _,
                offset: _,
            } = param.transform
            {
                sprite_world_offset + Vec2::from(dest)
            } else {
                sprite_world_offset
            };
            canvas.draw(image, param.dest(dest));
        };
    }
}
