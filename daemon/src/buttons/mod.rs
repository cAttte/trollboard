use std::collections::HashMap;
use serde::Serialize;

mod flip_screen;
mod lock_mouse;
mod swap_mouse;
mod win_key;

#[derive(Serialize)]
pub struct Button {
    pub name: &'static str,
    pub desc: &'static str,
    pub icon: &'static str,
    #[serde(skip)]
    pub run: fn(is_press: bool) -> Result<(), &'static str>,

}

inventory::collect!(Button);

pub fn vec() -> Vec<&'static Button> {
    inventory::iter::<Button>().collect()
}

pub fn map() -> HashMap<&'static str, &'static Button> {
    inventory::iter::<Button>().map(|b| (b.name, b)).collect()
}
