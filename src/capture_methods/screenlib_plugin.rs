use crate::screen_backgrounds::{Screen, ScreenLibCaptureTag};
use bevy::prelude::*;
use image::DynamicImage;
use rayon::prelude::*;
use screenshots::Screen as ScreenLib;

pub struct ScreenLibCapturePlugin;
impl Plugin for ScreenLibCapturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_screens);
    }
}

fn update_screens(
    mut query: Query<(&mut Screen, &Handle<Image>), With<ScreenLibCaptureTag>>,
    mut textures: ResMut<Assets<Image>>,
    time: Res<Time>,
) {
    // Cache the screens
    let all_screens = ScreenLib::all().unwrap();

    // Filter and collect the screens you're interested in, you can parallelize this part
    let relevant_screens: Vec<_> = all_screens
        .par_iter()
        .filter(|&libscreen| {
            query
                .iter()
                .any(|(screen, _)| libscreen.display_info.id == screen.id)
        })
        .collect();

    for (mut screen, texture) in &mut query {
        // tick the refresh rate timer
        screen.refresh_rate.tick(time.delta());
        // skip if not time to refresh
        if !screen.refresh_rate.finished() {
            continue;
        }

        // find the capturer for this screen
        let capturer = relevant_screens
            .iter()
            .find(|&libscreen| libscreen.display_info.id == screen.id);
        if capturer.is_none() {
            println!("No capturer found for screen {}", screen.name);
            continue;
        }

        let start = std::time::Instant::now();
        let image_buf = capturer.unwrap().capture().unwrap();
        print!("capture took {:?}", start.elapsed());

        let start2 = std::time::Instant::now();
        let dynamic_image = DynamicImage::ImageRgba8(image_buf);
        let image = Image::from_dynamic(dynamic_image, true);
        textures.get_mut(texture).unwrap().data = image.data;
        println!(
            " | texture took {:?} | total took {:?}",
            start2.elapsed(),
            start.elapsed()
        );
    }
}
