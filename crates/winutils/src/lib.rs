#![feature(let_chains)]

use bevy::math::IRect;

pub mod ui_automation;
pub mod win_colors;
pub mod win_keyboard;
pub mod win_mouse;
pub mod win_screen_capture;
pub mod win_wallpaper;
pub mod win_window;
pub mod win_events;

pub trait ToBevyIRect {
    fn to_bevy_irect(&self) -> IRect;
}
