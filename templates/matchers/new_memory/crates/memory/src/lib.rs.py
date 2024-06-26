# pub mod {{crate_name}}_memory_plugin;
# {{existing}}

def gather_variables(text: str) -> dict[str,str]:
    return {
        "existing": text,
    }

#region WORKSPACE CONTENT
#mod agent_observation_memory_plugin;
#mod main_camera_memory_plugin;
#mod main_character_memory_plugin;
#mod memory_plugin;
#pub mod primary_window_memory_plugin;
#mod ui_data_memory_plugin;
#mod voice_to_text_memory_plugin;
#
#pub mod prelude {
#    pub use crate::memory_plugin::*;
#    pub use cursor_hero_memory_types;
#}
#
#endregion

