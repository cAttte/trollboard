use macros::button;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

#[button(desc = "Press Windows key")]
fn win_key(is_press: bool) -> Result<(), &'static str> {
    if is_press {
        unsafe {
            let down = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_LWIN,
                        ..Default::default()
                    },
                },
            };

            let mut up = down.clone();
            up.Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;

            SendInput(&[down, up], std::mem::size_of::<INPUT>() as i32);
        }
    }

    Ok(())
}
