use cursor_hero_winutils::win_wallpaper::get_transcoded_wallpaper_path;
use cursor_hero_winutils::win_wallpaper::get_wallpaper_path;
use std::path::PathBuf;

/// Will update the wallpaper used in the game environment
fn main() {
    let wallpaper_path = get_wallpaper_path();
    if let Err(e) = wallpaper_path {
        panic!("Error: {:?}", e);
    }
    let mut wallpaper_path = PathBuf::from(wallpaper_path.unwrap());
    println!("Wallpaper path: {:?}", wallpaper_path);
    // check it exists
    if !wallpaper_path.exists() {
        println!("Wallpaper path does not exist, attempting transcoded path");
        wallpaper_path = match get_transcoded_wallpaper_path() {
            Ok(path) => {
                println!("Wallpaper path: {:?}", path);
                // check it exists
                if !path.exists() {
                    panic!("Wallpaper path does not exist");
                }
                path
            }
            Err(e) => panic!("Error: {:?}", e),
        };
    }
    // copy to assets/textures/environment/game/wallpaper.png
    let dest = std::path::Path::new("assets/textures/environment/game/wallpaper.png");
    std::fs::copy(wallpaper_path, dest).unwrap();
}
