use macros::button;
use windows::Win32::UI::Input::KeyboardAndMouse::SwapMouseButton;
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_SWAPBUTTON};

#[button(desc = "Swap mouse buttons")]
fn swap_mouse(_: bool) -> Result<(), &'static str> {
    unsafe {
        let currently_normal = GetSystemMetrics(SM_SWAPBUTTON) == 0;
        let _ = SwapMouseButton(currently_normal);
    }

    Ok(())
}
