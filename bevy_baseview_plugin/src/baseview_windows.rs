use bevy::utils::HashMap;
use bevy::window::{Window, WindowDescriptor, WindowId};
use raw_window_handle::HasRawWindowHandle;

use crate::{BaseviewWindow, BaseviewWindowInfo};

#[derive(Debug, Default)]
pub struct BaseviewWindows {
    pub windows: HashMap<WindowId, BaseviewWindow>,

    pub window_id_to_baseview: HashMap<WindowId, u64>,
    pub baseview_to_window_id: HashMap<u64, WindowId>,
    // Some window functions, such as `set_window_icon` can only be used from the main thread. If
    // they are used in another thread, the app will hang. This marker ensures `WinitWindows` is
    // only ever accessed with bevy's non-send functions and in NonSend systems.
    _not_send_sync: core::marker::PhantomData<*const ()>,
}

impl BaseviewWindows {
    pub fn create_window(
        &mut self,
        window_id: WindowId,
        window_descriptor: &WindowDescriptor,
        baseview_window_info: &BaseviewWindowInfo,
    ) -> Window {
        // TODO: Sort out scale factor.
        let scale_factor = 1.0;
        let window_info =
            baseview::WindowInfo::from_physical_size(baseview_window_info.phy_size, scale_factor);

        let window_open_options = baseview::WindowOpenOptions {
            title: "baseview".into(),
            size: window_info.logical_size(),
            scale: baseview::WindowScalePolicy::SystemScaleFactor,
        };

        let parent_win = &baseview_window_info.parent_win;

        let baseview_window = BaseviewWindow::new();
        let baseview_window_id = baseview_window.id();
        baseview::Window::open_parented(&parent_win, window_open_options, |_| baseview_window);
        self.baseview_to_window_id
            .insert(baseview_window_id, window_id);
        self.window_id_to_baseview
            .insert(window_id, baseview_window_id);

        Window::new(
            window_id,
            window_descriptor,
            baseview_window_info.phy_size.width,
            baseview_window_info.phy_size.height,
            scale_factor,
            None, // position,
            parent_win.raw_window_handle(),
        )
    }

    pub fn remove_window(&mut self, id: WindowId) -> Option<BaseviewWindow> {
        self.windows.remove(&id)
    }

    pub fn get_window_id(&self, id: u64) -> Option<WindowId> {
        self.baseview_to_window_id.get(&id).cloned()
    }
}
