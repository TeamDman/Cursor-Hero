use bevy::prelude::*;
use cursor_hero_plugins::prelude::*;
use cursor_hero_version::version_plugin::VersionPlugin;

enum LaunchMode {
    Default,
    Inspect,
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let launch_mode = match args.get(1).map(|s| s.as_str()) {
        Some("inspect") => LaunchMode::Inspect,
        _ => LaunchMode::Default,
    };

    let mut app = App::new();
    app.add_plugins(VersionPlugin(env!("CARGO_PKG_VERSION").to_string()));

    match launch_mode {
        LaunchMode::Default => {
            app.add_plugins(DefaultLaunchModePlugin);
        }
        LaunchMode::Inspect => {
            app.add_plugins(InspectLaunchModePlugin);
        }
    }
    app.run();
}
