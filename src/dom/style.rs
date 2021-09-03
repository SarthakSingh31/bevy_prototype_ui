use bevy::{math::prelude::*, render2::color::Color};

#[derive(Debug, Clone)]
pub enum Background {
    Color(Color),
    Transparent,
    // TODO: Add texture
    // TODO: Add material
}

#[derive(Debug, Clone)]
pub struct Styles {
    pub size: Vec2,
    pub margin: Vec2,
    pub background: Background,
}
