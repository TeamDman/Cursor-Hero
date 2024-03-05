use bevy::math::IRect;
use bevy::math::IVec2;
use bevy::math::IVec3;
use bevy::math::Rect;
use bevy::math::Vec2;
use bevy::math::Vec3;

pub trait NegativeYRect {
    fn neg_y(&self) -> Rect;
}
impl NegativeYRect for Rect {
    fn neg_y(&self) -> Rect {
        Rect::from_center_size(self.center().neg_y(), self.size())
    }
}

pub trait NegativeYIRect {
    fn neg_y(&self) -> IRect;
}
impl NegativeYIRect for IRect {
    fn neg_y(&self) -> IRect {
        IRect::from_center_size(self.center().neg_y(), self.size())
    }
}

pub trait NegativeYVec2 {
    fn neg_y(&self) -> Vec2;
}
impl NegativeYVec2 for Vec2 {
    fn neg_y(&self) -> Vec2 {
        Vec2::new(self.x, -self.y)
    }
}

pub trait NegativeYIVec2 {
    fn neg_y(&self) -> IVec2;
}
impl NegativeYIVec2 for IVec2 {
    fn neg_y(&self) -> IVec2 {
        IVec2::new(self.x, -self.y)
    }
}

pub trait NegativeYVec3 {
    fn neg_y(&self) -> Vec3;
}
impl NegativeYVec3 for Vec3 {
    fn neg_y(&self) -> Vec3 {
        Vec3::new(self.x, -self.y, self.z)
    }
}

pub trait NegativeYIVec3 {
    fn neg_y(&self) -> IVec3;
}
impl NegativeYIVec3 for IVec3 {
    fn neg_y(&self) -> IVec3 {
        IVec3::new(self.x, -self.y, self.z)
    }
}
