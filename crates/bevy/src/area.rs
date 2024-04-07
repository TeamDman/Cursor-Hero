use bevy::math::IRect;
use bevy::math::IVec2;
use bevy::math::Rect;
use bevy::math::URect;
use bevy::math::UVec2;
use bevy::math::Vec2;

pub trait Area {
    fn area(&self) -> usize;
}
impl Area for Vec2 {
    fn area(&self) -> usize {
        (self.x * self.y) as usize
    }
}
impl Area for IVec2 {
    fn area(&self) -> usize {
        (self.x * self.y) as usize
    }
}
impl Area for UVec2 {
    fn area(&self) -> usize {
        (self.x * self.y) as usize
    }
}
impl Area for Rect {
    fn area(&self) -> usize {
        self.size().area()
    }
}
impl Area for IRect {
    fn area(&self) -> usize {
        self.size().area()
    }
}
impl Area for URect {
    fn area(&self) -> usize {
        self.size().area()
    }
}
