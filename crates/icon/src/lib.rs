use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::winit::WinitWindows;
use winit::window::Icon;

pub struct IconPlugin;

impl Plugin for IconPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_window_icon);
    }
}

#[derive(Resource)]
struct WindowIconResource(Handle<Image>, SystemId);

fn load_window_icon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let icon_handle = asset_server.load("textures/icon.png");
    commands.add(|world: &mut World| {
        // register the system
        let system_id = world.register_system(update_window_icon);
        info!("Registered update_window_icon system with id {:?}", system_id);

        // add it to the update schedule
        let mut schedules =  world.resource_mut::<Schedules>();
        if let Some(schedule) = schedules.get_mut(Update) {
            schedule.add_systems(update_window_icon);
        } else {
            let mut new_schedule = Schedule::new(Update);
            new_schedule.add_systems(update_window_icon);
            schedules.insert(new_schedule);
        }

        // add handle holder with system id for later removal
        world.insert_resource(WindowIconResource(icon_handle, system_id));
    });
}

fn update_window_icon(
    windows: NonSend<WinitWindows>,
    materials: Res<Assets<Image>>,
    icon_resource: Res<WindowIconResource>,
    mut commands: Commands,
    mut flag: Local<bool>
) {
    if *flag {
        return;
    }
    if let Some(icon) = materials.get(&icon_resource.0) {
        // update the icon
        let icon = Icon::from_rgba(icon.clone().data, icon.size().x, icon.size().y).unwrap();
        for window in windows.windows.values() {
            window.set_window_icon(Some(icon.clone()));
        }
        info!("Updated window icon");

        // remove this system
        let system_id = icon_resource.1;
        commands.add(
            move |world: &mut World| {
                match world.remove_system(system_id) {
                    Ok(_) => info!("Removed update_window_icon system since it did its job"),
                    Err(e) => error!("Failed to remove update_window_icon system: {}", e),
                }
                let mut schedules =  world.resource_mut::<Schedules>();
                if let Some(schedule) = schedules.get_mut(Update) {
                    /*
https://discord.com/channels/691052431525675048/749335865876021248/1138225592064561243
Alice 🌹 — 08/07/2023 5:42 PM
We're also missing a Schedule::remove_system API, with a solution for
a) dependency invalidation and
b) disambiguation of multiple copies of a system in the same schedule
                     */

                    // this method does not exist yet.
                    // should probably make a PR for it
                    // and should also include a simplified way to remove a system from a schedule
                    // maybe a .remove_when similar to .run_if
                    // schedule.remove_system(system_id);
                }
            }
        );
        *flag = true;
    }
}
