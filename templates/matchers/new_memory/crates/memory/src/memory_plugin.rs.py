# {{before_memory_plugin}}
# use crate::{{crate_name}}_memory_plugin::{{crate_name_pascal}}MemoryPlugin;
# {{existing_stuff}}
#         if self.build_config.{{crate_name}}_memory_enabled {
#             app.add_plugins({{crate_name_pascal}}MemoryPlugin);
#         }
# {{end}}

def gather_variables(text: str) -> dict[str,str]:
    # before_memory_plugin
    find = "pub struct MemoryPlugin {"
    include = False
    index = text.find(find)
    assert index != -1, f"Coult not find `{find}`"
    index = index + len(find) if include else index
    before_memory_plugin, remaining = text[:index],text[index:]

    # existing_stuff
    find = "\n    }"
    include = False
    index = remaining.find(find)
    assert index != -1, f"Coult not find `{find}`"
    index = index + len(find) if include else index
    existing_stuff, remaining = remaining[:index],remaining[index:]

    # end
    end = remaining

    return {
        "before_memory_plugin": before_memory_plugin,
        "existing_stuff": existing_stuff,
        "end": end,
    }

#region WORKSPACE CONTENT
#use bevy::prelude::*;
#use cursor_hero_memory_types::prelude::MemoryConfig;
#use cursor_hero_memory_types::prelude::MemoryPluginBuildConfig;
#
#use crate::agent_observation_memory_plugin::AgentObservationMemoryPlugin;
#use crate::main_camera_memory_plugin::MainCameraMemoryPlugin;
#use crate::main_character_memory_plugin::MainCharacterMemoryPlugin;
#use crate::primary_window_memory_plugin::PrimaryWindowMemoryPlugin;
#use crate::ui_data_memory_plugin::UIDataMemoryPlugin;
#use crate::voice_to_text_memory_plugin::VoiceToTextMemoryPlugin;
#
#pub struct MemoryPlugin {
#    pub config: MemoryConfig,
#    pub build_config: MemoryPluginBuildConfig,
#}
#
#impl Plugin for MemoryPlugin {
#    fn build(&self, app: &mut App) {
#        app.insert_resource(self.config.clone());
#        if self.build_config.main_character_memory_enabled {
#            app.add_plugins(MainCharacterMemoryPlugin);
#        }
#        if self.build_config.primary_window_memory_enabled {
#            app.add_plugins(PrimaryWindowMemoryPlugin);
#        }
#        if self.build_config.main_camera_memory_enabled {
#            app.add_plugins(MainCameraMemoryPlugin);
#        }
#        if self.build_config.voice_to_text_memory_enabled {
#            app.add_plugins(VoiceToTextMemoryPlugin);
#        }
#        if self.build_config.agent_observation_memory_enabled {
#            app.add_plugins(AgentObservationMemoryPlugin);
#        }
#        if self.build_config.ui_data_memory_enabled {
#            app.add_plugins(UIDataMemoryPlugin);
#        }
#    }
#}
#
#endregion

