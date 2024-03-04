use bevy::math::Rect;
use bevy::math::Vec2;

pub trait AtInsideBottom {
    fn at_inside_bottom(&self, other: &Rect) -> Rect;
}
impl AtInsideBottom for Rect {
    fn at_inside_bottom(&self, other: &Rect) -> Rect {
        Rect::from_center_size(
            Vec2::new(
                other.center().x,
                other.center().y - other.height() / 2.0 + self.height() / 2.0,
            ),
            self.size(),
        )
    }
}
