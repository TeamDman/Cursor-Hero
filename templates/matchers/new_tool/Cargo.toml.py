# {{before_first_workspace_dependency}}
# cursor_hero_{{crate_name}}_tool = { path = "./crates/{{crate_name}}_tool" }
# {{first_workspace_dependency_onwards}}



def gather_variables(text: str) -> dict[str,str]:
    # before_first_workspace_dependency
    find = "[workspace.dependencies]"
    include = True
    index = text.find(find)
    assert index != -1, f"Coult not find `{find}`"
    index = index + len(find) if include else index
    before_first_workspace_dependency, remaining = text[:index],text[index:]

    # first_workspace_dependency_onwards
    first_workspace_dependency_onwards = remaining

    return {
        "before_first_workspace_dependency": before_first_workspace_dependency,
        "first_workspace_dependency_onwards": first_workspace_dependency_onwards,
    }

##### OLD CONTENT OF THIS FILE

# # {{before_first_workspace_dependency}}
# # cursor_hero_{{crate_name}}_tool = { path = "./crates/{{crate_name}}_tool" }
# # {{first_workspace_dependency_onwards}
# 
# 
# 
# def gather_variables(text: str) -> dict[str,str]:
#     find = "some part of the file"
#     include = True
#     index = text.find(find)
#     assert index != -1, f"Coult not find `{find}`"
#     index = index + len(find) if include else index
#     before_first_workspace_dependency, remaining = text[:index],text[index:]
# 
#     first_workspace_dependency_onwards = remaining
# 
#     return {
#         "before_first_workspace_dependency": before_first_workspace_dependency,
#         "first_workspace_dependency_onwards": first_workspace_dependency_onwards,
#     }
# 
# ##### OLD CONTENT OF THIS FILE
# 
# # # {{before_first_workspace_dependency}}
# # # cursor_hero_{{crate_name}}_tool = { path = "./crates/{{crate_name}}_tool" }
# # # {{first_workspace_dependency_onwards}
# # 
# # 
# # 
# # def gather_variables(text: str) -> dict[str,str]:
# #     find = "some part of the file"
# #     include = True
# #     index = text.find(find)
# #     assert index != -1, f"Coult not find `{find}`"
# #     index = index + len(find) if include else index
# #     before_first_workspace_dependency, remaining = text[:index],text[index:]
# # 
# #     first_workspace_dependency_onwards = remaining
# #     return {
# #         "before_first_workspace_dependency": before_first_workspace_dependency,
# #         "first_workspace_dependency_onwards": first_workspace_dependency_onwards,
# #     }
# # 
# # ##### OLD CONTENT OF THIS FILE
# # 
# # # # {{before_first_workspace_dependency}}
# # # # cursor_hero_{{crate_name}}_tool = { path = "./crates/{{crate_name}}_tool" }
# # # # {{first_workspace_dependency_onwards}
# # # 
# # # 
# # # 
# # # def gather_variables(text: str) -> dict[str,str]:
# # #     find = "some part of the file"
# # #     include = True
# # #     index = text.find(find)
# # #     assert index != -1, f"Coult not find `{find}`"
# # #     index = index + len(find) if include else index
# # #     before_first_workspace_dependency, remaining = text[:index],text[index:]
# # # 
# # #     find = "some part of the file"
# # #     include = True
# # #     index = remaining.find(find)
# # #     assert index != -1, f"Coult not find `{find}`"
# # #     index = index + len(find) if include else index
# # #     first_workspace_dependency_onwards, remaining = remaining[:index],remaining[index:]
# # # 
# # #     return {
# # #         "before_first_workspace_dependency": before_first_workspace_dependency,
# # #         "first_workspace_dependency_onwards": first_workspace_dependency_onwards,
# # #     }
# # # 
# # # ##### OLD CONTENT OF THIS FILE
# # # 
# # # # # {{before_first_workspace_dependency}}
# # # # # cursor_hero_{{crate_name}}_tool = { path = "./crates/{{crate_name}}_tool" }
# # # # # {{first_workspace_dependency_onwards}
# # # # 
# # # # 
# # # # 
# # # # def gather_variables(text: str) -> dict[str,str]:
# # # #     find = "some part of the file"
# # # #     include = True
# # # #     index = text.find(find)
# # # #     assert index != -1, f"Coult not find `{find}`"
# # # #     index = index + len(find) if include else index
# # # #     before_first_workspace_dependency, remaining = text[:index],text[index:]
# # # # 
# # # #     find = "some part of the file"
# # # #     include = True
# # # #     index = remaining.find(find)
# # # #     assert index != -1, f"Coult not find `{find}`"
# # # #     index = index + len(find) if include else index
# # # #     first_workspace_dependency_onwards, remaining = remaining[:index],remaining[index:]
# # # # 
# # # #     return {
# # # #         "before_first_workspace_dependency": before_first_workspace_dependency,
# # # #         "first_workspace_dependency_onwards": first_workspace_dependency_onwards,
# # # #     }
# # # # 
# # # # ##### OLD CONTENT OF THIS FILE
# # # # 
# # # # # # {{before_first_workspace_dependency}}
# # # # # # cursor_hero_{{crate_name}}_tool = { path = "./crates/{{crate_name}}_tool" }
# # # # # # {{first_workspace_dependency_onwards}
# # # # # 
# # # # # 
# # # # # 
# # # # # def gather_variables(text: str) -> dict[str,str]:
# # # # #     find = "some part of the file"
# # # # #     include = True
# # # # #     index = text.find(find)
# # # # #     assert index != -1, f"Coult not find `{find}`"
# # # # #     before_first_workspace_dependency, remaining = text[:index],text[index:]
# # # # # 
# # # # #     find = "some part of the file"
# # # # #     include = True
# # # # #     index = remaining.find(find)
# # # # #     assert index != -1, f"Coult not find `{find}`"
# # # # #     first_workspace_dependency_onwards, remaining = remaining[:index],remaining[index:]
# # # # # 
# # # # #     return {
# # # # #         "before_first_workspace_dependency": before_first_workspace_dependency,
# # # # #         "first_workspace_dependency_onwards": first_workspace_dependency_onwards,
# # # # #     }
# # # # # 
# # # # # ##### OLD CONTENT OF THIS FILE
# # # # # 
# # # # # # # {{before_first_workspace_dependency}}
# # # # # # # cursor_hero_{{crate_name}}_tool = { path = "./crates/{{crate_name}}_tool" }
# # # # # # # {{first_workspace_depndency_onwards}}
# # # # # # 
# # # # # # 
# # # # # # 
# # # # # # def gather_variables(text: str) -> dict[str,str]:
# # # # # #     find = "some part of the file"
# # # # # #     include = true
# # # # # #     index = text.find(find)
# # # # # #     assert index != -1, f"Coult not find `{find}`"
# # # # # #     before_first_workspace_dependency, remaining = text[:index],text[index:]
# # # # # # 
# # # # # #     find = "some part of the file"
# # # # # #     include = true
# # # # # #     index = remaining.find(find)
# # # # # #     assert index != -1, f"Coult not find `{find}`"
# # # # # #     first_workspace_dependency_onwards, remaining = remaining[:index],remaining[index:]
# # # # # # 
# # # # # #     return {
# # # # # #         "before_first_workspace_dependency": before_first_workspace_dependency,
# # # # # #         "first_workspace_dependency_onwards": first_workspace_dependency_onwards,
# # # # # #     }
# # # # # # 
# # # # # # ##### OLD CONTENT OF THIS FILE
# # # # # # 
# # # # # # # # {{before_first_workspace_dependency}}
# # # # # # # # cursor_hero_{{crate_name}}_tool = { path = "./crates/{{crate_name}}_tool" }
# # # # # # # # {{first_workspace_dependency_onwards}
# # # # # # # 
# # # # # # # 
# # # # # # # 
# # # # # # # def gather_variables(text: str) -> dict[str,str]:
# # # # # # #     find = "some part of the file"
# # # # # # #     include = true    index = text.find(find)
# # # # # # #     assert index != -1, f"Coult not find `{find}`"
# # # # # # #     before_first_workspace_dependency, remaining = text[:index],text[index:]
# # # # # # # 
# # # # # # #     find = "some part of the file"
# # # # # # #     include = true    index = remaining.find(find)
# # # # # # #     assert index != -1, f"Coult not find `{find}`"
# # # # # # #     first_workspace_dependency_onwards, remaining = remaining[:index],remaining[index:]
# # # # # # # 
# # # # # # #     return {
# # # # # # #         "before_first_workspace_dependency": before_first_workspace_dependency,
# # # # # # #         "first_workspace_dependency_onwards": first_workspace_dependency_onwards,
# # # # # # #     }
# # # # # # # 
# # # # # # # ##### OLD CONTENT OF THIS FILE
# # # # # # # 
# # # # # # # # # {{before_first_workspace_dependency}}
# # # # # # # # # cursor_hero_{{crate_name}}_tool = { path = "./crates/{{crate_name}}_tool" }
# # # # # # # # # {{first_workspace_dependency_onw
# # # # # # # # 
# # # # # # # # 
# # # # # # # # 
# # # # # # # # def gather_variables(text: str) -> dict[str,str]:
# # # # # # # #     find = "some part of the file"
# # # # # # # #     include = true    index = text.find(find)
# # # # # # # #     assert index != -1, f"Coult not find `{find}`"
# # # # # # # #     before_first_workspace_dependency, remaining = text[:index],text[index:]
# # # # # # # # 
# # # # # # # #     find = "some part of the file"
# # # # # # # #     include = true    index = remaining.find(find)
# # # # # # # #     assert index != -1, f"Coult not find `{find}`"
# # # # # # # #     first_workspace_dependency_onwards, remaining = remaining[:index],remaining[index:]
# # # # # # # # 
# # # # # # # #     return {
# # # # # # # #         "before_first_workspace_dependency": before_first_workspace_dependency,
# # # # # # # #         "first_workspace_dependency_onwards": first_workspace_dependency_onwards,
# # # # # # # #     }
# # # # # # # # 
# # # # # # # # ##### OLD CONTENT OF THIS FILE# # {{before_first_workspace_dependency}}
# # # # # # # # # # cursor_hero_{{crate_name}}_tool = { path = "./crates/{{crate_name}}_tool" }
# # # # # # # # # 
# # # # # # # # # 
# # # # # # # # # 
# # # # # # # # # 
# # # # # # # # # def gather_variables(text: str) -> dict[str,str]:
# # # # # # # # #     find = "some part of the file"
# # # # # # # # #     include = true    index = text.find(find)
# # # # # # # # #     assert index != -1, f"Coult not find `{find}`"
# # # # # # # # #     before_first_workspace_dependency, remaining = text[:index],text[index:]
# # # # # # # # # 
# # # # # # # # #     find = "some part of the file"
# # # # # # # # #     include = true    index = remaining.find(find)
# # # # # # # # #     assert index != -1, f"Coult not find `{find}`"
# # # # # # # # #     first_workspace_dependency_onwards, remaining = remaining[:index],remaining[index:]
# # # # # # # # # 
# # # # # # # # #     return {
# # # # # # # # #         "before_first_workspace_dependency": before_first_workspace_dependency,
# # # # # # # # #         "first_workspace_dependency_onwards": first_workspace_dependency_onwards,
# # # # # # # # #     }
# # # # # # # # 
# # # # # # # # 
# # # # # # # 
# # # # # # # 
# # # # # # 
# # # # # # 
# # # # # 
# # # # # 
# # # # 
# # # # 
# # # 
# # # 
# # 
# # 
# 
# 


##### WORKSPACE CONTENT
#[package]
#name = "cursor_hero"
#version = "0.3.0"
#edition = "2021"
#
#[workspace]
#members = ["crates/*", "other/gamepad_hell", "other/uparrow-enter"]
#
## See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
#
#[workspace.dependencies]
#cursor_hero_environment_types = { path = "./crates/environment_types" }
#cursor_hero_chat_types = { path = "./crates/chat_types" }
#cursor_hero_chat = { path = "./crates/chat" }
#cursor_hero_tts = { path = "./crates/tts" }
#cursor_hero_tts_types = { path = "./crates/tts_types" }
#cursor_hero_observation_types = { path = "./crates/observation_types" }
#cursor_hero_observation = { path = "./crates/observation" }
#cursor_hero_inference_types = { path = "./crates/inference_types" }
#cursor_hero_inference = { path = "./crates/inference" }
#cursor_hero_movement_tool_types = { path = "./crates/movement_tool_types" }
#cursor_hero_movement_tool = { path = "./crates/movement_tool" }
#cursor_hero_character_types = { path = "./crates/character_types" }
#cursor_hero_agent_types = { path = "./crates/agent_types" }
#cursor_hero_physics_debug = { path = "./crates/physics_debug" }
#cursor_hero_math = { path = "./crates/math" }
#cursor_hero_cursor_types = { path = "./crates/pointer_types" }
#cursor_hero_toolbelt_types = { path = "./crates/toolbelt_types" }
#cursor_hero_sprint_tool_types = { path = "./crates/sprint_tool_types" }
#cursor_hero_sprint_tool = { path = "./crates/sprint_tool" }
#cursor_hero_agent = { path = "./crates/agent" }
#cursor_hero_taskbar = { path = "./crates/taskbar" }
#cursor_hero_environment_nametag = { path = "./crates/environment_nametag" }
#cursor_hero_environment = { path = "./crates/environment" }
#cursor_hero_xelu_prompts = { path = "./crates/xelu_prompts" }
#cursor_hero_pause_tool = { path = "./crates/pause_tool" }
#cursor_hero_wallpaper = { path = "./crates/wallpaper" }
#cursor_hero_cursor_mirror = { path = "./crates/cursor_mirror" }
#cursor_hero_hover = { path = "./crates/hover" }
#cursor_hero_pressure_plate = { path = "./crates/pressure_plate" }
#cursor_hero_level_bounds = { path = "./crates/level_bounds" }
#cursor_hero_bevy = { path = "./crates/bevy" }
#cursor_hero_input = { path = "./crates/input" }
#cursor_hero_camera = { path = "./crates/camera" }
#cursor_hero_character = { path = "./crates/character" }
#cursor_hero_data = { path = "./crates/data" }
#cursor_hero_metrics = { path = "./crates/metrics" }
#cursor_hero_click_drag_character_movement = { path = "./crates/click_drag_character_movement" }
#cursor_hero_physics = { path = "./crates/physics" }
#cursor_hero_cursor = { path = "./crates/pointer" }
#cursor_hero_screen = { path = "./crates/screen" }
#cursor_hero_toolbelt = { path = "./crates/toolbelt" }
#cursor_hero_tools = { path = "./crates/tools" }
#cursor_hero_ui = { path = "./crates/ui" }
#cursor_hero_winutils = { path = "./crates/winutils" }
#cursor_hero_restart_memory = { path = "./crates/restart_memory" }
#cursor_hero_version = { path = "./crates/version" }
#cursor_hero_plugins = { path = "./crates/plugins" }
#cursor_hero_icon = { path = "./crates/icon" }
##inline_tweak = {git = "https://github.com/Uriopass/inline_tweak", version = "1.1.0", tag = "v1.1.0"} # features=["derive"]
#anyhow = "1.0.75"
#glam = "0.25.0"
## bevy = { path = "../bevy", features = ["dynamic_linking"] }
#bevy = { version = "0.12.1", git = "https://github.com/TeamDman/bevy.git", branch = "cursor_hero" }
#bevy-inspector-egui = { version = "0.22.1", git = "https://github.com/TeamDman/bevy-inspector-egui.git", branch = "cursor_hero" }
#bevy_xpbd_2d = { git = "https://github.com/TeamDman/bevy_xpbd.git", branch = "cursor_hero", features = [
#  "simd",
#  "parallel",
#] }
#bevy_egui = { git = "https://github.com/TeamDman/bevy_egui.git", branch = "cursor_hero", version = "0.24" }
## enigo = {git = "https://github.com/TeamDman/enigo", branch = "cursor_hero", version = "0.1.3"}
## enigo = {path = "../../rust/enigo", version = "0.1.3"}
#enigo = "0.2.0-rc2"
#egui = "0.24"
#crossbeam-channel = "0.5.8"
#itertools = "0.12.0"
#raw-window-handle = "0.5.2"
#leafwing-input-manager = { git = "https://github.com/TeamDman/leafwing-input-manager.git", branch = "cursor_hero" }
#windows = "0.51.1"
#indexmap = "2.1.0"
#image = "0.24.7"
#screenshots = "0.8.4"
#uiautomation = "0.7.3"
#serde = { version = "1.0", features = ["derive"] }
#serde_json = "1.0"
#tokio = { version = "1.32.0", features = ["net", "full"] }
#tokio-named-pipes = "0.1.0"
#syn = { version = "2.0.48", features = ["full", "visit-mut", "visit"] }
#quote = "1.0.35"
#proc-macro2 = "1.0.76"
#bevy_embedded_assets = { version = "0.9.1", git = "https://github.com/TeamDman/bevy_embedded_assets.git", branch = "cursor_hero" }
#winit = "0.28.7"
#fxhash = "0.2.1"
#widestring = "1.0.2"
#winreg = "0.52.0"
#rand = "0.8.5"
#ollama-rs = "0.1.6"
#reqwest = "0.11.24"
#urlencoding = "2.1.3"
#chrono = "0.4.33"
#
#[dependencies]
#cursor_hero_plugins = { workspace = true }
#cursor_hero_version = { workspace = true }
#bevy = { workspace = true }
#
#
#[dev-dependencies]
#bevy = { workspace = true, features = ["dynamic_linking"] }
#
#
#[build-dependencies]
#embed-resource = "1.6.3"
#
#[profile.dev]
#opt-level = 1
#
#[profile.dev.package."*"]
#opt-level = 3
#
## [patch.crates-io]
## winit = { path = "D:/Repos/rust/winit" }
#