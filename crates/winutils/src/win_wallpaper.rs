use std::env;
use std::ffi::OsString;
use std::os::raw::c_void;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use windows::Win32::UI::WindowsAndMessaging::SystemParametersInfoW;
use windows::Win32::UI::WindowsAndMessaging::SPIF_UPDATEINIFILE;
use windows::Win32::UI::WindowsAndMessaging::SPI_GETDESKWALLPAPER;
/// This gets the path to the file at the time the user set the wallpaper.
/// The file may have moved since then.
pub fn get_wallpaper_path() -> Result<OsString, windows::core::Error> {
    let mut buffer = vec![0u16; 32768]; // theoretical max path after increase
    unsafe {
        SystemParametersInfoW(
            SPI_GETDESKWALLPAPER,
            buffer.len() as u32,
            Some(buffer.as_mut_ptr() as *mut c_void),
            SPIF_UPDATEINIFILE,
        )
    }?;

    // Find the position of the first null character
    let len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());

    // Create a slice of the buffer up to the found position
    let trimmed_buffer = &buffer[..len];

    Ok(OsString::from_wide(trimmed_buffer))
}

pub fn get_transcoded_wallpaper_path() -> Result<PathBuf, env::VarError> {
    let app_data = env::var("APPDATA")?; // Get the value of the APPDATA environment variable
    let themes_path =
        PathBuf::from(app_data).join("Microsoft\\Windows\\Themes\\TranscodedWallpaper");
    Ok(themes_path)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_wallpaper_path() {
        let path = super::get_wallpaper_path();
        println!("Wallpaper path: {:?}", path);
        assert!(path.is_ok());
    }

    #[test]
    fn test_get_transcoded_wallpaper_path() {
        let path = super::get_transcoded_wallpaper_path();
        println!("Transcoded wallpaper path: {:?}", path);
        assert!(path.is_ok());
    }
}
