use macros::button;
use windows::Win32::Foundation::{POINT, RECT};
use windows::Win32::Media::Audio::{PlaySoundW, SND_ALIAS, SND_ASYNC, SND_SYSTEM};
use windows::Win32::UI::WindowsAndMessaging::{ClipCursor, GetCursorPos};
use windows::core::w;

macro_rules! play_sound {
    ($alias:literal) => {
        let _ = PlaySoundW(w!($alias), None, SND_ALIAS | SND_ASYNC | SND_SYSTEM);
    };
}

#[button(desc = "Disable mouse cursor")]
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

            play_sound!("DeviceDisconnect");
            ClipCursor(Some(&rect)).map_err(|_| "Failed to lock cursor")?;
        }
    } else {
        unsafe {
            play_sound!("DeviceConnect");
            ClipCursor(None).map_err(|_| "Failed to unlock cursor")?;
        }
    }

    Ok(())
}
