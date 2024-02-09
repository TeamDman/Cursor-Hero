# {{before_first_plugins_dependency}}
# cursor_hero_{{crate_name}} = { workspace = true }
# {{first_plugins_dependency_onwards}}

from typing import Tuple

def chunk(text: str) -> Tuple[str, str]:
    # TODO: Implement logic for first_plugins_dependency_onwards
    # TODO: Implement logic for before_first_plugins_dependency
    return ()

##### WORKSPACE CONTENT
#[package]
#name = "cursor_hero_plugins"
#version = "0.1.0"
#edition = "2021"
#
## See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
#
#
#[dependencies]
#cursor_hero_environment_types = { workspace = true }
#cursor_hero_chat_types = { workspace = true }
#cursor_hero_chat = { workspace = true }
#cursor_hero_tts = { workspace = true }
#cursor_hero_tts_types = { workspace = true }
#cursor_hero_observation_types = { workspace = true }
#cursor_hero_observation = { workspace = true }
#cursor_hero_inference_types = { workspace = true }
#cursor_hero_inference = { workspace = true }
#cursor_hero_movement_tool_types = { workspace = true }
#cursor_hero_movement_tool = { workspace = true }
#cursor_hero_character_types = { workspace = true }
#cursor_hero_agent_types = { workspace = true }
#cursor_hero_physics_debug = { workspace = true }
#cursor_hero_math = { workspace = true }
#cursor_hero_version = { workspace = true }
#cursor_hero_pointer_types = { workspace = true }
#cursor_hero_toolbelt_types = { workspace = true }
#cursor_hero_sprint_tool_types = { workspace = true }
#cursor_hero_sprint_tool = { workspace = true }
#cursor_hero_agent = { workspace = true }
#cursor_hero_taskbar = { workspace = true }
#cursor_hero_environment_nametag = { workspace = true }
#cursor_hero_environment = { workspace = true }
#cursor_hero_xelu_prompts = { workspace = true }
#cursor_hero_pause_tool = { workspace = true }
#cursor_hero_wallpaper = { workspace = true }
#cursor_hero_cursor_mirror = {workspace = true}
#cursor_hero_hover = {workspace = true}
#cursor_hero_pressure_plate = {workspace = true}
#cursor_hero_level_bounds = {workspace = true}
#cursor_hero_input = {workspace = true}
#cursor_hero_camera = {workspace = true}
#cursor_hero_character = {workspace = true}
#cursor_hero_data = {workspace = true}
#cursor_hero_metrics = {workspace = true}
#cursor_hero_click_drag_character_movement = {workspace = true}
#cursor_hero_physics = {workspace = true}
#cursor_hero_pointer = {workspace = true}
#cursor_hero_screen = {workspace = true}
#cursor_hero_toolbelt = {workspace = true}
#cursor_hero_tools = {workspace = true}
#cursor_hero_ui = {workspace = true}
#cursor_hero_icon = {workspace = true}
#cursor_hero_winutils = {workspace = true}
#cursor_hero_restart_memory = {workspace = true}
#bevy = {workspace = true}
#bevy_embedded_assets = { workspace = true }
#bevy-inspector-egui = { workspace = true }
#
#[dev-dependencies]
#cursor_hero_restart_memory = {workspace = true}
#