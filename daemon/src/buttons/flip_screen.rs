use super::Button;
use macros::button;
use windows::Win32::Graphics::Gdi::{
    CDS_RESET, CDS_UPDATEREGISTRY, ChangeDisplaySettingsExA, DEVMODEA, DISP_CHANGE_SUCCESSFUL,
    DM_DISPLAYORIENTATION, DMDO_90, DMDO_180, DMDO_270, DMDO_DEFAULT, ENUM_CURRENT_SETTINGS,
    EnumDisplaySettingsA,
};
use windows::core::PCSTR;

#[button(desc = "Flip screen orientation")]
fn flip_screen(_: bool) -> Result<(), &'static str> {
    unsafe {
        let mut devmode = DEVMODEA::default();
        devmode.dmSize = std::mem::size_of::<DEVMODEA>() as u16;

        if EnumDisplaySettingsA(None, ENUM_CURRENT_SETTINGS, &mut devmode).as_bool() {
            devmode.dmFields = DM_DISPLAYORIENTATION;
            devmode.Anonymous1.Anonymous2.dmDisplayOrientation =
                match devmode.Anonymous1.Anonymous2.dmDisplayOrientation {
                    DMDO_DEFAULT => DMDO_180,
                    DMDO_180 => DMDO_DEFAULT,
                    DMDO_90 => DMDO_270,
                    DMDO_270 => DMDO_90,
                    other => other,
                };

            let result = ChangeDisplaySettingsExA(
                PCSTR::null(), // primary display
                Some(&devmode),
                None,
                CDS_UPDATEREGISTRY | CDS_RESET,
                None,
            );

            match result {
                DISP_CHANGE_SUCCESSFUL => Ok(()),
                _ => Err("Failed to change screen orientation"),
            }
        } else {
            Err("Failed to get current display settings")
        }
    }
}
