use windows::Win32::Foundation::POINT;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

fn main() {
    loop {
        let cursor_pos = get_cursor_position().expect("Failed to get cursor position");
        println!("Cursor position: {:?}", cursor_pos);
    }
}

fn get_cursor_position() -> Result<POINT, windows::core::Error> {
    unsafe {
        let mut point = POINT::default();
        GetCursorPos(&mut point)?;
        Ok(point)
    }
}
