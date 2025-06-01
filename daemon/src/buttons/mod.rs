use serde::Serialize;
pub mod flip_screen;
pub mod lock_mouse;
pub mod win_key;

pub const ALL: &'static [Button] = &[win_key::BUTTON, flip_screen::BUTTON, lock_mouse::BUTTON];

#[derive(Serialize)]
pub struct Button {
    pub name: &'static str,
    pub desc: &'static str,
    pub icon: &'static str,
    #[serde(skip)]
    pub run: fn(is_press: bool) -> Result<(), &'static str>,
}
