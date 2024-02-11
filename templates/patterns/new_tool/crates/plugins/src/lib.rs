{{use_statements}}
use cursor_hero_{{crate_name}}_tool::prelude::*;
{{plugin_start}}
        app.add_plugins({{crate_name_pascal}}ToolPlugin);
        {{plugin_remaining}}