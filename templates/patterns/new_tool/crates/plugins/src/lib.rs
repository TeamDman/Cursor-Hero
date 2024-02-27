{{use_statements}}
use cursor_hero_{{crate_name}}_tool::prelude::*;
use cursor_hero_{{crate_name}}_tool_types::prelude::*;
{{plugin_start}}
        app.add_plugins({{crate_name_pascal}}ToolPlugin);
        app.add_plugins({{crate_name_pascal}}ToolTypesPlugin);
        {{plugin_remaining}}