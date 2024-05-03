use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use image::DynamicImage;
use image::RgbImage;

use crate::screen_plugin::Screen;

#[derive(Debug)]
pub enum GetImageError {
    ElementEmpty,
}

#[derive(SystemParam)]
pub struct ScreensToImageParam<'w, 's> {
    pub images: Res<'w, Assets<Image>>,
    pub screens: Query<'w, 's, (&'static Handle<Image>, &'static GlobalTransform), With<Screen>>,
}

pub trait ImageHolder {
    fn get_image_buffer(
        &self,
        bounds: IRect,
    ) -> Result<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, GetImageError>;
    fn get_image(&self, bounds: IRect) -> Result<Image, GetImageError>;
}

impl ImageHolder for ScreensToImageParam<'_, '_> {
    fn get_image_buffer(
        &self,
        bounds: IRect,
    ) -> Result<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, GetImageError> {
        if bounds.is_empty() {
            return Err(GetImageError::ElementEmpty);
        }
        let mut tex = RgbImage::new(bounds.width() as u32, bounds.height() as u32);

        // find out what parts of each screen are intersecting with the element
        for (screen_image_handle, screen_trans) in self.screens.iter() {
            // find out the image size
            let screen_center_pos = screen_trans.translation();
            match self.images.get(screen_image_handle) {
                None => {}
                Some(screen_image) => {
                    // Calculate the overlapping area
                    let screen_size = screen_image.texture_descriptor.size;
                    let mut screen_origin = screen_center_pos.xy();
                    screen_origin.y *= -1.0;
                    let screen_rect = Rect::from_center_size(
                        screen_origin,
                        Vec2::new(screen_size.width as f32, screen_size.height as f32),
                    );

                    // find the overlap
                    // debug!("screen_rect: {:?}", screen_rect);
                    let intersection = screen_rect.intersect(bounds.as_rect());
                    // debug!("intersection rect: {:?}", intersection);

                    // convert to monitor coordinates
                    let origin = intersection.center() - screen_rect.min.xy();
                    let tex_grab_rect = Rect::from_center_size(origin, intersection.size());
                    // debug!("tex_grab_rect: {:?}", tex_grab_rect);

                    if !tex_grab_rect.is_empty() {
                        // debug!(
                        //     "Copying pixel range {} by {}",
                        //     tex_grab_rect.size().x,
                        //     tex_grab_rect.size().y
                        // );

                        // Calculate where to start placing pixels in the element's texture
                        let texture_start_x = (intersection.min.x - bounds.min.x as f32) as u32;
                        let texture_start_y = (intersection.min.y - bounds.min.y as f32) as u32;
                        // debug!("Texture start: {} {}", texture_start_x, texture_start_y);
                        // Copy the overlapping part of the screen texture to the element's texture.
                        for y in tex_grab_rect.min.y as usize..tex_grab_rect.max.y as usize {
                            for x in tex_grab_rect.min.x as usize..tex_grab_rect.max.x as usize {
                                let start = (y * screen_size.width as usize + x) * 4;
                                if start + 4 <= screen_image.data.len() {
                                    let pixel: [u8; 3] = [
                                        screen_image.data[start],
                                        screen_image.data[start + 1],
                                        screen_image.data[start + 2],
                                        // screen_image.data[start + 3],
                                    ];
                                    tex.put_pixel(
                                        texture_start_x + x as u32 - tex_grab_rect.min.x as u32,
                                        texture_start_y + y as u32 - tex_grab_rect.min.y as u32,
                                        image::Rgb(pixel),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(tex)
    }

    fn get_image(&self, bounds: IRect) -> Result<Image, GetImageError> {
        let tex = self.get_image_buffer(bounds)?;
        let dynamic_image = DynamicImage::ImageRgb8(tex);
        let image = Image::from_dynamic(dynamic_image, true);
        Ok(image)
    }
}

pub trait AsBevyColor {
    fn as_bevy_color(&self) -> Color;
}

impl AsBevyColor for image::Rgb<u8> {
    fn as_bevy_color(&self) -> Color {
        Color::rgb(
            self[0] as f32 / 255.0,
            self[1] as f32 / 255.0,
            self[2] as f32 / 255.0,
        )
    }
}
