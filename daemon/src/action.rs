use serde::Deserialize;

use crate::buttons;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    button: String,
    is_press: bool,
}

impl Action {
    pub fn run(self) {
        let button = buttons::ALL.iter().find(|b| b.name == self.button);
        if let Some(button) = button {
            // TODO: handle and communicate errors
            let _ = (button.run)(self.is_press);
        }
    }
}
