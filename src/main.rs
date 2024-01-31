use bevy::prelude::*;
use cursor_hero_plugins::MyPlugin;
use cursor_hero_version::version_plugin::VersionPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(VersionPlugin(env!("CARGO_PKG_VERSION").to_string()));
    app.add_plugins(MyPlugin);
    // .insert_resource(ClearColor(Color::NONE))
    app.run();
}
