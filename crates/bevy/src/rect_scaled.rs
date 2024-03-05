use bevy::math::IRect;
use bevy::math::Rect;
use bevy::math::URect;
use bevy::math::Vec2;

pub trait RectScale {
    fn scale(&self, scale: Vec2) -> Rect;
}
impl RectScale for Rect {
    fn scale(&self, scale: Vec2) -> Rect {
        Rect {
            min: self.min * scale,
            max: self.max * scale,
        }
    }
}

pub trait IRectScale {
    fn scale(&self, scale: Vec2) -> IRect;
}
impl IRectScale for IRect {
    fn scale(&self, scale: Vec2) -> IRect {
        IRect {
            min: (self.min.as_vec2() * scale).as_ivec2(),
            max: (self.max.as_vec2() * scale).as_ivec2(),
        }
    }
}

pub trait URectScale {
    fn scale(&self, scale: Vec2) -> URect;
}
impl URectScale for URect {
    fn scale(&self, scale: Vec2) -> URect {
        URect {
            min: (self.min.as_vec2() * scale).as_uvec2(),
            max: (self.max.as_vec2() * scale).as_uvec2(),
        }
    }
}
