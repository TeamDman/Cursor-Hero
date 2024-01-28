use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use bevy::prelude::*;
use cursor_hero_character::character_plugin::MainCharacter;

pub struct MainCharacterMemoryPlugin;

impl Plugin for MainCharacterMemoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                restore_main_character_position,
                remember_main_character_position,
            ),
        );
    }
}

enum Vec3ParseError {
    InvalidFormat,
    ParseFloatError(std::num::ParseFloatError),
}
fn parse_vec3(s: &str) -> Result<bevy::math::Vec3, Vec3ParseError> {
    let parts: Vec<&str> = s
        .trim_matches(|c| c == '[' || c == ']')
        .split(',')
        .collect();

    if parts.len() == 3 {
        let x = parts[0]
            .trim()
            .parse::<f32>()
            .map_err(Vec3ParseError::ParseFloatError)?;
        let y = parts[1]
            .trim()
            .parse::<f32>()
            .map_err(Vec3ParseError::ParseFloatError)?;
        let z = parts[2]
            .trim()
            .parse::<f32>()
            .map_err(Vec3ParseError::ParseFloatError)?;

        Ok(bevy::math::Vec3::new(x, y, z))
    } else {
        Err(Vec3ParseError::InvalidFormat)
    }
}

fn restore_main_character_position(
    mut character_query: Query<&mut Transform, Added<MainCharacter>>,
) {
    let Ok(mut character_transform) =  character_query.get_single_mut() else {
        // No new character, nothing to do
        return;
    };

    let dir = Path::new(file!()).parent().expect("No parent directory");
    let file_path = dir.join("main_character_position.txt");

    // Read from the file
    let input = fs::read_to_string(file_path);
    let content = match input {
        Ok(content) => content,
        Err(_) => {
            warn!("Couldn't read the position file or it doesn't exist.");
            return;
        }
    };
    let lines = content.trim_end().lines().collect::<Vec<_>>();

    // Check if there are two lines (timestamp and position)
    if lines.len() != 2 {
        warn!(
            "File format is incorrect. Found content with {} lines: {}",
            lines.len(),
            content
        );
        return;
    }

    // Parse the timestamp and position
    let saved_timestamp = lines[0].parse::<u64>().unwrap_or(0);
    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Check if the saved timestamp is less than an hour old
    if current_timestamp - saved_timestamp > 3600 {
        warn!("Saved position is more than an hour old, not restoring.");
        return;
    }

    // Parse the saved position
    let saved_position = match parse_vec3(lines[1]) {
        Ok(position) => position,
        Err(_) => {
            warn!("Couldn't parse the saved position.");
            return;
        }
    };

    // Update the character position
    info!("Restoring main character position to {:?}", saved_position);
    character_transform.translation = saved_position;
}

fn remember_main_character_position(
    character_query: Query<&Transform, With<MainCharacter>>,
    mut last_checkpoint: Local<Duration>,
    mut last_saved_position: Local<Vec3>,
) {
    // make sure it has been 1 second since the last checkpoint
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let time_since_last_checkpoint = since_the_epoch - *last_checkpoint;
    if time_since_last_checkpoint < Duration::from_secs(1) {
        return;
    }

    let character_transform = match character_query.get_single() {
        Ok(character_transform) => character_transform,
        Err(_) => {
            warn!("Couldn't find main character transform");
            return;
        }
    };
    if character_transform.translation == *last_saved_position {
        return;
    }
    let character_position = character_transform.translation;

    // Get the directory of the current file
    let dir = Path::new(file!()).parent().expect("No parent directory");

    // Construct the path for the new file in the same directory
    let file_path = dir.join("main_character_position.txt");

    // Write to the file at the specified path
    let mut file = match File::create(&file_path) {
        Ok(file) => file,
        Err(_) => {
            warn!("Couldn't create {:?}", file_path);
            return;
        }
    };

    if let Err(e) = writeln!(
        file,
        "{}\n{}",
        since_the_epoch.as_secs(),
        character_position.to_string()
    ) {
        warn!("Couldn't write to {:?}: {}", file_path, e);
        return;
    }

    *last_checkpoint = since_the_epoch;
    *last_saved_position = character_position;
}
