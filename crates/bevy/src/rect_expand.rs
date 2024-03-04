use bevy::math::IRect;
use bevy::math::IVec2;

use cursor_hero_math::prelude::Corner;
pub trait IExpandable {
    fn expand(&self, amount: IVec2) -> IRect;
    fn expand_from(&self, corner: Corner, amount: IVec2) -> IRect;
}
impl IExpandable for IRect {
    fn expand(&self, amount: IVec2) -> IRect {
        IRect::from_center_size(self.center(), self.size() + amount)
    }
    fn expand_from(&self, corner: Corner, amount: IVec2) -> IRect {
        match corner {
            Corner::TopLeft => IRect{
                min: self.min - amount, 
                max: self.max,
            },
            Corner::TopRight => IRect{
                min: self.min - IVec2::new(0, amount.y),
                max: self.max + IVec2::new(amount.x, 0),
            },
            Corner::BottomLeft => IRect{
                min: self.min - IVec2::new(amount.x, 0),
                max: self.max + IVec2::new(0, amount.y),
            },
            Corner::BottomRight => IRect{
                min: self.min,
                max: self.max + amount,
            }
        }
    }
}
