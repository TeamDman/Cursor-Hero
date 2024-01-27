use bevy::math::{IRect, IVec2};
use screenshots::display_info::DisplayInfo;

pub mod get_image;
pub mod screen_plugin;
pub mod screen_update_plugin;

pub trait ToBevyIRect {
    fn to_bevy_irect(&self) -> IRect;
}
impl ToBevyIRect for DisplayInfo {
    fn to_bevy_irect(&self) -> IRect {
        IRect {
            min: IVec2::new(self.x, self.y),
            max: IVec2::new(self.x + self.width as i32, self.y + self.height as i32),
        }
    }
}
