use bevy::prelude::*;
use bevy_egui::EguiUserTextures;
use cursor_hero_bevy::prelude::Area;
use cursor_hero_bevy::prelude::TopLeftI;
use cursor_hero_bevy::prelude::TranslateIVec2;
use cursor_hero_screen::get_image::ImageHolder;
use cursor_hero_screen::get_image::ScreensToImageParam;
use cursor_hero_ui_automation::prelude::DrillId;
use cursor_hero_ui_inspector_types::prelude::PreviewImage;
use cursor_hero_ui_inspector_types::prelude::UIData;
use image::DynamicImage;
use image::Rgb;

pub struct UiInspectorPreviewImagePlugin;

impl Plugin for UiInspectorPreviewImagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_preview_image
                .run_if(|ui_data: Res<UIData>| ui_data.opened.global_toggle && ui_data.opened.tree),
        );
    }
}

fn update_preview_image(
    screen_access: ScreensToImageParam,
    asset_server: Res<AssetServer>,
    mut ui_data: ResMut<UIData>,
    mut debounce: Local<Option<DrillId>>,
    mut egui_user_textures: ResMut<EguiUserTextures>,
) {
    // Avoid duplicate work
    if *debounce == ui_data.selected {
        return;
    }
    debounce.clone_from(&ui_data.selected);
    let image = (|| {
        // Determine what to preview
        let (info, parent_info) = match ui_data.selected.clone() {
            Some(DrillId::Child(ref inner)) => {
                // Get parent ID by dropping last element
                let mut parent_drill_id = inner.clone();
                parent_drill_id.pop_back();
                let parent_drill_id = match parent_drill_id.len() {
                    0 => DrillId::Root,
                    _ => DrillId::Child(parent_drill_id),
                };

                // Look up info for parent
                let Some(parent_info) = ui_data.ui_tree.lookup_drill_id(parent_drill_id.clone())
                else {
                    warn!("Failed to find parent info for {:?}", parent_drill_id);
                    return None;
                };

                // Look up info
                let Some(last) = inner.back() else {
                    warn!("Failed to find last element in {:?}", inner);
                    return None;
                };
                let Some(info) = parent_info.lookup_drill_id([last].into_iter().cloned().collect())
                else {
                    warn!("Failed to find info for {:?}", inner);
                    return None;
                };
                (info, parent_info)
            }
            Some(DrillId::Root) => {
                let info = &ui_data.ui_tree;
                let parent_info = &ui_data.ui_tree;
                (info, parent_info)
            }
            Some(DrillId::Unknown) => {
                warn!("Selected drill_id is unknown");
                return None;
            }
            None => return None,
        };

        // Calculate regions
        let world_capture_region = match parent_info.drill_id {
            DrillId::Root => info.children.as_ref().map_or_else(
                || info.bounding_rect,
                |children| {
                    children
                        .iter()
                        .fold(info.bounding_rect, |acc, x| acc.union(x.bounding_rect))
                },
            ),
            DrillId::Child(_) => {
                if parent_info.bounding_rect.is_empty() {
                    info.bounding_rect
                } else {
                    parent_info.bounding_rect
                }
            }
            DrillId::Unknown => {
                warn!("Parent drill_id is unknown");
                return None;
            }
        };
        let texture_highlight_region = info
            .bounding_rect
            .translated(&-world_capture_region.top_left());
        let size = world_capture_region.size().abs().as_uvec2();

        // Check assumptions about reasonable image sizes given my personal monitor setup
        // This fn is running on the main thread so big operations will lag the UI
        if size.area() > IVec2::new(2100 * 3, 1100).area() {
            warn!(
                "Image size is very large: {:?} ({} sq px), skipping",
                size,
                size.area()
            );
            return None;
        } else if size.area() == 0 {
            warn!("Image size is zero, skipping");
            return None;
        }

        // Get the texture of the element
        let Ok(mut image) = screen_access.get_image_buffer(world_capture_region) else {
            warn!("Failed to get image for region {:?}", world_capture_region);
            return None;
        };

        // Apply the highlight
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            if texture_highlight_region.contains(IVec2::new(x as i32, y as i32)) {
                *pixel = Rgb([
                    pixel.0[0].saturating_add(50),
                    pixel.0[1].saturating_add(50),
                    pixel.0[2],
                ]);
            }
        }

        // Convert back to Bevy image
        let image = Image::from_dynamic(DynamicImage::ImageRgb8(image), true);
        Some((image, size))
    })();
    if let Some((image, size)) = image {
        // Remove the old handle
        if let Some(ref preview) = ui_data.selected_preview {
            egui_user_textures.remove_image(&preview.handle);
        }
        // Register the handle with egui
        let handle = asset_server.add(image);
        egui_user_textures.add_image(handle.clone());
        ui_data.selected_preview = Some(PreviewImage { handle, size });
    } else {
        ui_data.selected_preview = None;
    }
}
