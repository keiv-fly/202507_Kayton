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

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    mod unix {
        use super::*;
        use std::io::Read;
        use std::os::unix::io::{FromRawFd, RawFd};

        /// Capture output written to STDOUT by temporarily redirecting it to a pipe.
        fn capture_stdout<F: FnOnce()>(func: F) -> Vec<u8> {
            unsafe {
                let mut fds: [RawFd; 2] = [0; 2];
                assert_eq!(libc::pipe(fds.as_mut_ptr()), 0, "pipe failed");

                let stdout_fd = libc::STDOUT_FILENO;
                let saved_fd = libc::dup(stdout_fd);
                assert!(saved_fd >= 0, "dup failed");

                assert_eq!(libc::dup2(fds[1], stdout_fd), stdout_fd, "dup2 failed");
                libc::close(fds[1]);

                func();

                assert_eq!(libc::dup2(saved_fd, stdout_fd), stdout_fd, "restore failed");
                libc::close(saved_fd);

                let mut file = std::fs::File::from_raw_fd(fds[0]);
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).unwrap();
                buffer
            }
        }

        #[test]
        fn test_print_to_console() {
            let msg = b"hello";
            let out = capture_stdout(|| print_to_console(msg));
            assert_eq!(out.as_slice(), msg);
        }

        #[test]
        fn test_println_to_console() {
            let msg = b"hello";
            let mut expected = Vec::new();
            expected.extend_from_slice(msg);
            expected.extend_from_slice(NL);
            let out = capture_stdout(|| println_to_console(msg));
            assert_eq!(out, expected);
        }
    }

    #[cfg(windows)]
    mod windows {
        use super::*;

        #[test]
        fn test_print_to_console() {
            // On Windows, simply ensure the function does not panic.
            print_to_console(b"hello");
        }

        #[test]
        fn test_println_to_console() {
            println_to_console(b"hello");
        }
    }
}
