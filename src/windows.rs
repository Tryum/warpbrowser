use windows::Win32::System::SystemInformation::{GetVersionExW, OSVERSIONINFOW};

pub fn get_windows_version() -> Option<(u32, u32, u32)> {
    let mut os_info = OSVERSIONINFOW {
        dwOSVersionInfoSize: std::mem::size_of::<OSVERSIONINFOW>() as u32,
        ..Default::default()
    };

    match unsafe { GetVersionExW(&mut os_info) } {
        Ok(_) => Some((
            os_info.dwMajorVersion,
            os_info.dwMinorVersion,
            os_info.dwBuildNumber,
        )),
        Err(err) => {
            tracing::error!("Failed to get windows version with error : {}", err);
            None
        }
    }
}
