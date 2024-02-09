{{use_statements}}
use cursor_hero_{{crate_name}}::prelude::*;
use cursor_hero_{{crate_name}}_types::prelude::*;
{{plugin_start}}
        app.add_plugins({{crate_name_pascal}}TypesPlugin);
        app.add_plugins({{crate_name_pascal}}Plugin);
{{plugin_remaining}}