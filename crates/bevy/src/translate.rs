use bevy::math::IRect;
use bevy::math::IVec2;
use bevy::math::IVec3;
use bevy::math::Rect;
use bevy::math::Vec2;
use bevy::math::Vec3;

// Vecs
pub trait TranslateVec2 {
    fn translated(&self, translation: &Vec2) -> Self;
}
pub trait TranslateIVec2 {
    fn translated(&self, translation: &IVec2) -> Self;
}
pub trait TranslateVec3 {
    fn translated(&self, translation: &Vec3) -> Self;
}
pub trait TranslateIVec3 {
    fn translated(&self, translation: &IVec3) -> Self;
}

impl TranslateVec2 for Rect {
    fn translated(&self, translation: &Vec2) -> Rect {
        Rect {
            min: self.min + *translation,
            max: self.max + *translation,
        }
    }
}
impl TranslateIVec2 for IRect {
    fn translated(&self, translation: &IVec2) -> IRect {
        IRect {
            min: self.min + *translation,
            max: self.max + *translation,
        }
    }
}

impl TranslateVec2 for Vec2 {
    fn translated(&self, translation: &Vec2) -> Vec2 {
        *self + *translation
    }
}
impl TranslateIVec2 for IVec2 {
    fn translated(&self, translation: &IVec2) -> IVec2 {
        *self + *translation
    }
}

impl TranslateVec3 for Vec3 {
    fn translated(&self, translation: &Vec3) -> Vec3 {
        *self + *translation
    }
}
impl TranslateIVec3 for IVec3 {
    fn translated(&self, translation: &IVec3) -> IVec3 {
        *self + *translation
    }
}
