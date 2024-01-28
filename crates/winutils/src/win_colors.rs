use image::Rgba;
use std::error::Error;
use winreg::enums::*;
use winreg::RegKey;

pub fn get_accent_color() -> Result<Rgba<u8>, Box<dyn Error>> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let personalization =
        hklm.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Accent")?;
    let accent_color: u32 = personalization.get_value("AccentColorMenu")?;
    Ok(abgr_to_rgba(accent_color))
}

fn abgr_to_rgba(abgr: u32) -> Rgba<u8> {
    let a = ((abgr >> 24) & 0xff) as u8;
    let b = ((abgr >> 16) & 0xff) as u8;
    let g = ((abgr >> 8) & 0xff) as u8;
    let r = (abgr & 0xff) as u8;

    Rgba([r, g, b, a])
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_accent_color() {
        match super::get_accent_color() {
            Ok(color) => println!("Accent color: {:?}", color),
            Err(e) => panic!("Error reading accent color: {}", e),
        }
    }
}
