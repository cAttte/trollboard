use super::Button;
use macros::button;
use windows::Win32::Foundation::{POINT, RECT};
use windows::Win32::UI::WindowsAndMessaging::{ClipCursor, GetCursorPos};

#[button(desc = "Lock mouse cursor")]
fn lock_mouse(is_press: bool) -> Result<(), &'static str> {
    if is_press {
        let mut pos = POINT::default();
        unsafe {
            GetCursorPos(&mut pos).map_err(|_| "Failed to get cursor position")?;

            let rect = RECT {
                left: pos.x,
                top: pos.y,
                right: pos.x + 1,
                bottom: pos.y + 1,
            };

            ClipCursor(Some(&rect)).map_err(|_| "Failed to lock cursor")?;
        }
    } else {
        unsafe {
            ClipCursor(None).map_err(|_| "Failed to unlock cursor")?;
        }
    }

    Ok(())
}
