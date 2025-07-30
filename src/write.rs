#[cfg(windows)]
mod platform {
    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::System::Console::{GetStdHandle, STD_OUTPUT_HANDLE};
    use windows_sys::Win32::Storage::FileSystem::WriteFile;

    pub fn print_to_console(message: &[u8]) {
        let handle: HANDLE = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
        let mut written = 0u32;

        unsafe {
            WriteFile(
                handle,
                message.as_ptr(),
                message.len() as u32,
                &mut written as *mut u32,
                std::ptr::null_mut(),
            );
        }
    }
}

#[cfg(unix)]
mod platform {
    use libc;
    pub fn print_to_console(message: &[u8]) {
        unsafe {
            libc::write(libc::STDOUT_FILENO, message.as_ptr() as *const _, message.len());
        }
    }
}

// Re-export the unified interface
pub use platform::print_to_console;

#[cfg(windows)]
const NL: &[u8] = b"\r\n";
#[cfg(not(windows))]
const NL: &[u8] = b"\n";

pub fn println_to_console(message: &[u8]) {
    let mut buffer = Vec::with_capacity(message.len() + NL.len());
    buffer.extend_from_slice(message);
    buffer.extend_from_slice(NL);
    print_to_console(&buffer);
}