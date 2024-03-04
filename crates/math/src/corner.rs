use bevy::math::IRect;
use bevy::math::IVec2;
use bevy::reflect::Reflect;

#[derive(Debug, Reflect, Eq, Clone, PartialEq, Hash)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
impl Corner {
    pub fn variants() -> [Self; 4] {
        [Self::TopLeft, Self::TopRight, Self::BottomLeft, Self::BottomRight]
    }
    pub fn of_rect(rect: IRect) -> Self {
        match rect {
            IRect {
                min: IVec2 { x: 0, y: 0 },
                ..
            } => Self::TopLeft,
            IRect {
                min: IVec2 { x: 0, y: _ },
                ..
            } => Self::BottomLeft,
            IRect {
                min: IVec2 { x: _, y: 0 },
                ..
            } => Self::TopRight,
            IRect {
                min: IVec2 { x: _, y: _ },
                ..
            } => Self::BottomRight,
        }
    }
}
