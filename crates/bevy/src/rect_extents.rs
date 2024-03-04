use bevy::math::IRect;
use bevy::math::IVec2;
use bevy::math::Rect;
use bevy::math::Vec2;
use cursor_hero_math::prelude::Corner;

pub trait CornerOfRect {
    fn of(&self, rect: &Rect) -> Vec2;
}
impl CornerOfRect for Corner {
    fn of(&self, rect: &Rect) -> Vec2 {
        match self {
            Corner::TopLeft => rect.top_left(),
            Corner::TopRight => rect.top_right(),
            Corner::BottomLeft => rect.bottom_left(),
            Corner::BottomRight => rect.bottom_right(),
        }
    }
}

pub trait CornerOfIRect {
    fn of(&self, rect: &IRect) -> IVec2;
}
impl CornerOfIRect for Corner {
    fn of(&self, rect: &IRect) -> IVec2 {
        match self {
            Corner::TopLeft => rect.top_left(),
            Corner::TopRight => rect.top_right(),
            Corner::BottomLeft => rect.bottom_left(),
            Corner::BottomRight => rect.bottom_right(),
        }
    }
}

pub trait TopRight {
    fn top_right(&self) -> Vec2;
}
impl TopRight for Rect {
    fn top_right(&self) -> Vec2 {
        Vec2::new(self.max.x, self.min.y)
    }
}

pub trait TopRightI {
    fn top_right(&self) -> IVec2;
}
impl TopRightI for IRect {
    fn top_right(&self) -> IVec2 {
        IVec2::new(self.max.x, self.min.y)
    }
}

pub trait BottomLeft {
    fn bottom_left(&self) -> Vec2;
}
impl BottomLeft for Rect {
    fn bottom_left(&self) -> Vec2 {
        Vec2::new(self.min.x, self.max.y)
    }
}

pub trait BottomLeftI {
    fn bottom_left(&self) -> IVec2;
}
impl BottomLeftI for IRect {
    fn bottom_left(&self) -> IVec2 {
        IVec2::new(self.min.x, self.max.y)
    }
}

pub trait BottomRight {
    fn bottom_right(&self) -> Vec2;
}
impl BottomRight for Rect {
    fn bottom_right(&self) -> Vec2 {
        self.max
    }
}

pub trait BottomRightI {
    fn bottom_right(&self) -> IVec2;
}
impl BottomRightI for IRect {
    fn bottom_right(&self) -> IVec2 {
        self.max
    }
}

pub trait TopLeft {
    fn top_left(&self) -> Vec2;
}
impl TopLeft for Rect {
    fn top_left(&self) -> Vec2 {
        self.min
    }
}

pub trait TopLeftI {
    fn top_left(&self) -> IVec2;
}
impl TopLeftI for IRect {
    fn top_left(&self) -> IVec2 {
        self.min
    }
}

pub trait Left {
    fn left(&self) -> f32;
}
impl Left for Rect {
    fn left(&self) -> f32 {
        self.min.x
    }
}

pub trait LeftI {
    fn left(&self) -> i32;
}
impl LeftI for IRect {
    fn left(&self) -> i32 {
        self.min.x
    }
}

pub trait Right {
    fn right(&self) -> f32;
}
impl Right for Rect {
    fn right(&self) -> f32 {
        self.max.x
    }
}

pub trait RightI {
    fn right(&self) -> i32;
}
impl RightI for IRect {
    fn right(&self) -> i32 {
        self.max.x
    }
}

pub trait Bottom {
    fn bottom(&self) -> f32;
}
impl Bottom for Rect {
    fn bottom(&self) -> f32 {
        self.min.y
    }
}

pub trait BottomI {
    fn bottom(&self) -> i32;
}
impl BottomI for IRect {
    fn bottom(&self) -> i32 {
        self.min.y
    }
}

pub trait Top {
    fn top(&self) -> f32;
}
impl Top for Rect {
    fn top(&self) -> f32 {
        self.max.y
    }
}

pub trait TopI {
    fn top(&self) -> i32;
}
impl TopI for IRect {
    fn top(&self) -> i32 {
        self.max.y
    }
}