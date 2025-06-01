#[macro_export]
macro_rules! sudo {
    ($exe:expr) => {
        crate::sudo!($exe, "")
    };
    ($exe:literal, $args:expr $(,)?) => {{
        use windows::{
            core::w,
            Win32::UI::Shell::ShellExecuteW,
            Win32::UI::WindowsAndMessaging::SW_HIDE,
        };

        unsafe {
            ShellExecuteW(
                None,
                w!("runas"),
                w!($exe),
                w!($args),
                None,
                SW_HIDE,
            )
        };

        // if result.0 <= 32 {
        //     Err(std::io::Error::new(std::io::ErrorKind::Other, format!("ShellExecuteW failed with code {}", result.0)))
        // } else {
        //     Ok(())
        // }
    }};
}
