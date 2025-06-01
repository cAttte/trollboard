use serde::Serialize;
pub mod flip_screen;
pub mod lock_mouse;
pub mod swap_mouse;
pub mod win_key;

pub const ALL: &'static [Button] = &[flip_screen::BUTTON, lock_mouse::BUTTON, swap_mouse::BUTTON, win_key::BUTTON];

#[derive(Serialize)]
pub struct Button {
    pub name: &'static str,
    pub desc: &'static str,
    pub icon: &'static str,
    #[serde(skip)]
    pub run: fn(is_press: bool) -> Result<(), &'static str>,
}
