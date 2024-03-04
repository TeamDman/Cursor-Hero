use bevy::math::Rect;
use bevy::math::Vec2;

pub trait RectWithHeight {
    fn with_height(&self, height: f32) -> Rect;
}
impl RectWithHeight for Rect {
    fn with_height(&self, height: f32) -> Rect {
        Rect::from_center_size(self.center(), Vec2::new(self.width(), height))
    }
}
