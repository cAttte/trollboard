use super::Button;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

pub const BUTTON: Button = Button {
    name: "win_key",
    desc: "Press Windows key",
    icon: include_str!("../../icons/winkey.svg"),
    run: run,
};

fn run(is_press: bool) -> Result<(), &'static str> {
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

// unsafe {
//     let mut downs = vec![];
//     let mut ups = vec![];

//     for vk in &[VK_CONTROL, VK_ESCAPE] {
//         let down = INPUT {
//             r#type: INPUT_KEYBOARD,
//             Anonymous: INPUT_0 {
//                 ki: KEYBDINPUT {
//                     wVk: *vk,
//                     ..Default::default()
//                 },
//             },
//         };

//         let mut up = down.clone();
//         up.Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;

//         downs.push(down);
//         ups.push(up);
//     }

//     let inputs = [downs, ups].concat();
//     SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
// }
