{{before_memory_plugin}}
use crate::{{crate_name}}_memory_plugin::{{crate_name_pascal}}MemoryPlugin;
{{existing_stuff}}
        if self.build_config.{{crate_name}}_memory_enabled {
            app.add_plugins({{crate_name_pascal}}MemoryPlugin);
        }
{{end}}