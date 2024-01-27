#![feature(let_chains)]

use bevy::math::IRect;

pub mod win_keyboard;
pub mod win_mouse;
pub mod win_screen_capture;
pub mod win_window;
pub mod ui_automation;

pub trait ToBevyIRect {
    fn to_bevy_irect(&self) -> IRect;
}