use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

#[derive(Clone, Debug)]
pub struct ParentWin(RawWindowHandle);

impl ParentWin {
    pub fn new(ptr: *mut std::os::raw::c_void) -> Self {
        let handle = if cfg!(target_os = "macos") {
            let mut handle = raw_window_handle::AppKitHandle::empty();
            handle.ns_view = ptr;
            RawWindowHandle::AppKit(handle)
        } else if cfg!(target_os = "windows") {
            let mut handle = raw_window_handle::Win32Handle::empty();
            handle.hwnd = ptr;
            RawWindowHandle::Win32(handle)
        } else if cfg!(target_os = "linux") {
            let mut handle = raw_window_handle::XcbHandle::empty();
            handle.window = ptr as u32;
            RawWindowHandle::Xcb(handle)
        } else {
            panic!("Unsupported operating system: {}", std::env::consts::OS)
        };
        Self(handle)
    }
}

unsafe impl HasRawWindowHandle for ParentWin {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.0
    }
}

impl From<RawWindowHandle> for ParentWin {
    fn from(inst: RawWindowHandle) -> Self {
        ParentWin(inst)
    }
}
