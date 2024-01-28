use bevy::ecs::entity::Entity;
use bevy::math::IRect;
use bevy::math::IVec2;
use bevy::math::Rect;
use bevy::math::Vec2;
use bevy::prelude::Name;
use bevy::prelude::Vec3Swizzles;
use bevy::render::primitives::Aabb;

// Define a trait that provides a method to return a string from an Option<&Name>
pub trait NameOrEntityDisplay {
    fn name_or_entity(&self, entity: Entity) -> String;
}

// Implement the trait for Option<&Name>
impl NameOrEntityDisplay for Option<&Name> {
    fn name_or_entity(&self, entity: Entity) -> String {
        match self {
            Some(name) => name.to_string(),
            None => format!("Entity({:?})", entity),
        }
    }
}

pub trait IExpandable {
    fn expand(&self, amount: IVec2) -> IRect;
}
impl IExpandable for IRect {
    fn expand(&self, amount: IVec2) -> IRect {
        IRect::from_center_size(self.center(), self.size() + amount)
    }
}

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

pub trait AabbToRect {
    fn to_rect(&self) -> Rect;
    fn to_rect_with_offset(&self, offset: Vec2) -> Rect;
}
impl AabbToRect for Aabb {
    fn to_rect(&self) -> Rect {
        Rect::from_center_half_size(self.center.xy(), self.half_extents.xy())
    }
    fn to_rect_with_offset(&self, offset: Vec2) -> Rect {
        Rect::from_center_half_size(self.center.xy() + offset, self.half_extents.xy())
    }
}
