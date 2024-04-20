use bevy::prelude::*;
use cursor_hero_plugins::prelude::*;
use cursor_hero_version::version_plugin::VersionPlugin;

enum LaunchMode {
    Default,
    Headless,
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let launch_mode = match args.get(1).map(|s| s.as_str()) {
        Some("headless") => LaunchMode::Headless,
        None => LaunchMode::Default,
        x => {
            println!("Invalid launch mode: {:?}", x);
            return;
        }
    };

    let mut app = App::new();
    app.add_plugins(VersionPlugin(env!("CARGO_PKG_VERSION").to_string()));

    match launch_mode {
        LaunchMode::Default => {
            app.add_plugins(DefaultLaunchModePlugin);
        }
        LaunchMode::Headless => {
            app.add_plugins(HeadlessLaunchModePlugin);
        }
    }
    app.run();
}
