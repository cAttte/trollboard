use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    button: String,
    is_press: bool,
}

impl Action {
    pub fn run(self) {
        let map = crate::buttons::map();
        let button = map.get(self.button.as_str());
        if let Some(button) = button {
            // TODO: handle and communicate errors
            let _ = (button.run)(self.is_press);
        }
    }
}
