use std::path::Path;

pub fn format_tool_name_from_source(file_path: &str) -> String {
    // Extract the file name from the path
    let file_name = Path::new(file_path)
        .file_stem() // Get the file stem (file name without extension)
        .and_then(|stem| stem.to_str()) // Convert OsStr to &str
        .unwrap_or("");

    file_name
        .split('_')
        .map(|word| {
            word.chars()
                .enumerate()
                .map(|(i, c)| {
                    if i == 0 {
                        c.to_uppercase().to_string()
                    } else {
                        c.to_string()
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn format_tool_image_from_source(file_path: &str) -> String {
    // Extract the file name from the path
    let file_name = Path::new(file_path)
        .file_stem() // Get the file stem (file name without extension)
        .and_then(|stem| stem.to_str()) // Convert OsStr to &str
        .unwrap_or("");
    format!("textures/tools/{}.png", file_name)
}
