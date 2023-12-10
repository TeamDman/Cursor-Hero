use crate::capture_methods::inhouse::get_all_monitors;
use bevy::prelude::*;
use image::DynamicImage;
use screenshots::Screen as ScreenLib;
use std::collections::VecDeque;

pub struct ScreenPlugin;
impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_screens).add_systems(
            Update,
            (
                // update_screens,
                cycle_capture_method,
            ),
        )
        .register_type::<Screen>()
        .register_type::<ScreenParent>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Screen {
    pub id: u32,
    pub name: String,
    pub refresh_rate: Timer,
}

#[derive(Component, Reflect)]
pub struct ScreenParent;

fn spawn_screens(
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
    // mut capturer_resource: NonSendMut<CapturerResource>,
) {
    let mut parent = commands.spawn((
        SpatialBundle::default(),
        ScreenParent,
        Name::new("Screen Parent"),
    ));

    // create a Screen component for each screen
    let mut screen_names = get_all_monitors()
        .unwrap()
        .iter()
        .map(|monitor| monitor.info.name.clone())
        .collect::<VecDeque<String>>();

    parent.with_children(|parent| {
        for screen in ScreenLib::all().unwrap().iter() {
            let image_buf = screen.capture().unwrap();
            let dynamic_image = DynamicImage::ImageRgba8(image_buf);
            let image = Image::from_dynamic(dynamic_image, true);
            let texture = textures.add(image);
            let name = screen_names.pop_front().unwrap();
    
            parent.spawn((
                SpriteBundle {
                    texture,
                    transform: Transform::from_xyz(
                        screen.display_info.x as f32 + (screen.display_info.width as f32) / 2.0,
                        -(screen.display_info.y as f32) - (screen.display_info.height as f32) / 2.0,
                        -1.0,
                    ), // Position behind the character
                    ..Default::default()
                },
                Screen {
                    name: name.clone(),
                    id: screen.display_info.id,
                    refresh_rate: Timer::from_seconds(0.1, TimerMode::Repeating),
                },
                InhouseThreadedCaptureTag,
                Name::new(format!("Screen {}", name)),
            ));
    
        }
    });
}

#[derive(Component)]
pub struct ScreenLibCaptureTag;

#[derive(Component)]
pub struct InhouseCaptureTag;

#[derive(Component)]
pub struct InhouseThreadedCaptureTag;

#[derive(Component)]
pub struct WinDesktopDuplicationCaptureTag;

fn cycle_capture_method(
    mut commands: Commands,
    query_inhouse: Query<Entity, With<InhouseCaptureTag>>,
    query_inhouse_threaded: Query<Entity, With<InhouseThreadedCaptureTag>>,
    query_screen_lib: Query<Entity, With<ScreenLibCaptureTag>>,
    query_win_desktop: Query<Entity, With<WinDesktopDuplicationCaptureTag>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::M) {
        return;
    }

    // For entities with InhouseCaptureTag, switch to ScreenLibCaptureTag
    for entity in query_inhouse.iter() {
        commands
            .entity(entity)
            .remove::<InhouseCaptureTag>()
            .insert(InhouseThreadedCaptureTag);
        println!("Switched {:?} to InhouseThreadedCaptureTag", entity);
    }
    // For entities with InhouseCaptureTag, switch to ScreenLibCaptureTag
    for entity in query_inhouse_threaded.iter() {
        commands
            .entity(entity)
            .remove::<InhouseThreadedCaptureTag>()
            .insert(ScreenLibCaptureTag);
        println!("Switched {:?} to ScreenLibCaptureTag", entity);
    }

    // For entities with ScreenLibCaptureTag, switch to WinDesktopDuplicationCaptureTag
    for entity in query_screen_lib.iter() {
        commands
            .entity(entity)
            .remove::<ScreenLibCaptureTag>()
            .insert(WinDesktopDuplicationCaptureTag);
        println!("Switched {:?} to WinDesktopDuplicationCaptureTag", entity);
    }

    // For entities with WinDesktopDuplicationCaptureTag, switch to InhouseCaptureTag
    for entity in query_win_desktop.iter() {
        commands
            .entity(entity)
            .remove::<WinDesktopDuplicationCaptureTag>()
            .insert(InhouseCaptureTag);
        println!("Switched {:?} to InhouseCaptureTag", entity);
    }
}
