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
        baseview_window_info: BaseviewWindowInfo,
    ) -> Window {
        let BaseviewWindowInfo {
            window_open_options,
            parent_win,
        } = baseview_window_info;

        let baseview_window = BaseviewWindow::new();
        let baseview_window_id = baseview_window.id();

        let scale_factor = match window_open_options.scale {
            baseview::WindowScalePolicy::SystemScaleFactor => 1.0,
            baseview::WindowScalePolicy::ScaleFactor(scale) => scale,
        };

        let window_info =
            baseview::WindowInfo::from_logical_size(window_open_options.size, scale_factor);
        // TODO: Should probably add utilities to baseview to expose easier
        // scaling.
        let phy_size = window_open_options.size.to_physical(&window_info);

        baseview::Window::open_parented(&parent_win, window_open_options, |_| baseview_window);
        self.baseview_to_window_id
            .insert(baseview_window_id, window_id);
        self.window_id_to_baseview
            .insert(window_id, baseview_window_id);

        Window::new(
            window_id,
            window_descriptor,
            phy_size.width,
            phy_size.height,
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
