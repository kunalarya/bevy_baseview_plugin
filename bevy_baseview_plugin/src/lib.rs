//#![allow(unused_imports, dead_code, unused_mut, unused_variables)]
mod baseview_windows;
mod conversions;
mod default_plugins;
mod keyboard;
mod parent_win;

use bevy::input::ButtonState;
pub use parent_win::ParentWin;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use bevy::app::{App, CoreStage, Plugin};
use bevy::ecs::prelude::*;
use bevy::ecs::{
    event::{Events, ManualEventReader},
    world::World,
};
use bevy::input::{
    keyboard::KeyboardInput,
    mouse::{MouseButtonInput, MouseScrollUnit, MouseWheel},
};
use bevy::math::DVec2;
use bevy::window::{
    CreateWindow, CursorEntered, CursorLeft, CursorMoved, ModifiesWindows,
    WindowBackendScaleFactorChanged, WindowCreated, WindowFocused, WindowResized,
    WindowScaleFactorChanged, Windows,
};
use lazy_static::lazy_static;

use baseview_windows::BaseviewWindows;
pub use default_plugins::DefaultBaseviewPlugins;

/// Container for user-provided information about the parent window.
struct BaseviewWindowInfo {
    /// Physical size of parent window.
    //phy_size: baseview::PhySize,
    //scale_factor: baseview::WindowScalePolicy,
    parent_win: ParentWin,
    window_open_options: baseview::WindowOpenOptions,
}

impl std::fmt::Debug for BaseviewWindowInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Note, we do not support glContext for baseview.
        let window_open_options_str = format!(
            "WindowOpenOptions(title={:?}, size={:?}, scale={:?})",
            self.window_open_options.title,
            self.window_open_options.size,
            self.window_open_options.scale
        );
        f.debug_struct("BaseviewWindowInfo")
            .field("parent_win", &self.parent_win)
            .field("window_open_options", &window_open_options_str)
            .finish()
    }
}

impl Clone for BaseviewWindowInfo {
    fn clone(&self) -> Self {
        Self {
            parent_win: self.parent_win.clone(),
            window_open_options: clone_window_options(&self.window_open_options),
        }
    }
}

unsafe impl Sync for BaseviewWindowInfo {}
unsafe impl Send for BaseviewWindowInfo {}

/**
 * An AppProxy helps callers control the lifetime of the bevy app,
 * since it's stored statically and run within the window event loop
 * (which is triggered by the host). Dropping the AppProxy will drop
 * the bevy App.
 */
pub struct AppProxy;

impl Drop for AppProxy {
    fn drop(&mut self) {
        // TODO: GUI Thread is definitely not guaranteed; need to evaluate.
        drop_app(&GuiThread);
    }
}

/**
 * Attach a baseview window to an app.
 */
pub fn attach_to<P: Into<ParentWin>>(
    app: &mut App,
    window_open_options: &baseview::WindowOpenOptions,
    parent: P,
) -> AppProxy {
    let parent_win = parent.into();
    let window_open_options = clone_window_options(window_open_options);

    let baseview_window_info = BaseviewWindowInfo {
        parent_win,
        window_open_options,
    };
    app.insert_non_send_resource(baseview_window_info);

    AppProxy
}

// h/t to iced_baseview
fn clone_window_options(window: &baseview::WindowOpenOptions) -> baseview::WindowOpenOptions {
    baseview::WindowOpenOptions {
        title: window.title.clone(),
        #[cfg(feature = "baseviewgl")]
        gl_config: window
            .gl_config
            .as_ref()
            .map(|config| baseview::gl::GlConfig { ..*config }),
        ..*window
    }
}

#[derive(Default)]
pub struct BaseviewPlugin;

// Marker thread for indicating that something should only be called from the GUI thread. Not
// enforced, purely for internal documentation.
struct GuiThread;

impl Plugin for BaseviewPlugin {
    fn build(&self, app: &mut App) {
        app.init_non_send_resource::<BaseviewWindows>()
            .set_runner(baseview_runner)
            .add_system_to_stage(CoreStage::PostUpdate, change_window.label(ModifiesWindows));
        // #[cfg(target_arch = "wasm32")]
        // app.add_plugin(web_resize::CanvasParentResizePlugin);
        #[cfg(not(target_os = "android"))]
        let create_window_reader = BaseviewCreateWindowReader::default();
        app.insert_resource(create_window_reader);
        #[cfg(not(target_os = "android"))]
        handle_create_window_events(&mut app.world);
        app.add_event::<CloseAppRequest>();
    }
}

#[derive(Clone, Debug, Default)]
pub struct CloseAppRequest;

#[derive(Clone, Debug, Default)]
pub struct CloseAppResponse;

thread_local! {
    static APP: RefCell<Option<App>> = RefCell::new(None);
}

lazy_static! {
    static ref APP_SET: AtomicBool = AtomicBool::new(false);
}

fn baseview_runner(app: App) {
    // We have to store the App somewhere to prevent it from getting
    // dropped, so we store it in a thread-local, since App is not Send.
    //
    // This also allows us to access the app from the event loop.
    //
    // Since this runner will always run in the main thread, and should
    // only run once, that should be okay.
    //
    // This also means we're functionally making App 'static. Note that we have to verify that
    // multiple threads don't get store Apps, hence the lazy_static global atomic bool.
    APP.with(|app_rc| {
        if APP_SET
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            panic!("Cannot call baseview runner twice");
        }
        app_rc.replace(Some(app));
    });
}

fn drop_app(_gui: &GuiThread) {
    log::trace!("Dropping App");
    if APP_SET
        .compare_exchange(true, false, Ordering::Acquire, Ordering::Relaxed)
        .is_ok()
    {
        APP.with(|app_rc| {
            app_rc.replace(None);
        });
    }
    APP_SET.store(false, Ordering::Release);
}

lazy_static! {
    static ref BASEVIEW_WINDOW_ID: AtomicU64 = AtomicU64::new(0);
}

#[derive(Debug)]
pub struct BaseviewWindow {
    id: u64,
    last_scale_factor: f64,
    pending_events: VecDeque<baseview::Event>,
}

struct EventStatus {
    return_status: baseview::EventStatus,
    shutdown: bool,
}

impl BaseviewWindow {
    pub fn new() -> Self {
        let id = BASEVIEW_WINDOW_ID.fetch_add(1, Ordering::AcqRel);
        Self {
            id,
            last_scale_factor: 1.0,
            pending_events: VecDeque::new(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    fn process_pending_events(&mut self, app: &mut App) {
        while !self.pending_events.is_empty() {
            let pending_event = self.pending_events.pop_front().unwrap();
            let pending_status = self.process_event(app, pending_event);
            if pending_status.shutdown {
                log::warn!("ignored pending event shutdown request");
            }
        }
    }

    fn process_event(&mut self, app: &mut App, event: baseview::Event) -> EventStatus {
        let mut status = EventStatus {
            return_status: baseview::EventStatus::Captured,
            shutdown: false,
        };
        let world = app.world.cell();
        fn get_window_id(
            world: &bevy::ecs::world::WorldCell<'_>,
            baseview_window_id: u64,
        ) -> Option<bevy::window::WindowId> {
            let baseview_windows = world.non_send_resource_mut::<BaseviewWindows>();
            if let Some(window_id) = baseview_windows.get_window_id(baseview_window_id) {
                Some(window_id)
            } else {
                // TODO: Clean up logging.
                log::warn!(
                    "Skipped event for unknown baseview Window Id {:?}",
                    baseview_window_id
                );

                None
            }
        }

        // Check for shutdown requests.
        let mut close_app_requests = world.resource_mut::<Events<CloseAppRequest>>();
        for _ in close_app_requests.drain() {
            status.shutdown = true;
            let mut close_app_responses = world.resource_mut::<Events<CloseAppResponse>>();
            close_app_responses.send(CloseAppResponse);
        }

        match event {
            baseview::Event::Mouse(e) => {
                match e {
                    baseview::MouseEvent::CursorMoved { position, .. } => {
                        let mut cursor_moved_events = world.resource_mut::<Events<CursorMoved>>();
                        let window_id = if let Some(window_id) = get_window_id(&world, self.id()) {
                            window_id
                        } else {
                            return status;
                        };
                        let mut windows = world.resource_mut::<Windows>();
                        let window = if let Some(window) = windows.get_mut(window_id) {
                            window
                        } else {
                            // If we're here, this window was previously opened
                            log::info!("Skipped event for closed window: {:?}", window_id);
                            return status;
                        };
                        let position = DVec2::new(position.x, position.y);
                        window.update_cursor_physical_position_from_backend(Some(position));
                        cursor_moved_events.send(CursorMoved {
                            id: window_id,
                            position: position.as_vec2(),
                        });
                    }
                    baseview::MouseEvent::CursorEntered => {
                        let window_id = if let Some(window_id) = get_window_id(&world, self.id()) {
                            window_id
                        } else {
                            return status;
                        };
                        let mut cursor_entered_events =
                            world.resource_mut::<Events<CursorEntered>>();
                        cursor_entered_events.send(CursorEntered { id: window_id });
                    }
                    baseview::MouseEvent::ButtonPressed { button, .. } => {
                        let mut mouse_button_input_events =
                            world.resource_mut::<Events<MouseButtonInput>>();
                        mouse_button_input_events.send(MouseButtonInput {
                            button: conversions::baseview_mousebutton_to_bevy(button),
                            state: ButtonState::Pressed,
                        });
                    }
                    baseview::MouseEvent::ButtonReleased { button, .. } => {
                        let mut mouse_button_input_events =
                            world.resource_mut::<Events<MouseButtonInput>>();
                        mouse_button_input_events.send(MouseButtonInput {
                            button: conversions::baseview_mousebutton_to_bevy(button),
                            state: ButtonState::Released,
                        });
                    }
                    baseview::MouseEvent::CursorLeft => {
                        let window_id = if let Some(window_id) = get_window_id(&world, self.id()) {
                            window_id
                        } else {
                            return status;
                        };
                        let mut cursor_left_events = world.resource_mut::<Events<CursorLeft>>();
                        cursor_left_events.send(CursorLeft { id: window_id });
                    }
                    baseview::MouseEvent::WheelScrolled { delta, .. } => match delta {
                        baseview::ScrollDelta::Lines { x, y } => {
                            let mut mouse_wheel_input_events =
                                world.resource_mut::<Events<MouseWheel>>();
                            mouse_wheel_input_events.send(MouseWheel {
                                unit: MouseScrollUnit::Line,
                                x,
                                y,
                            });
                        }
                        baseview::ScrollDelta::Pixels { x, y } => {
                            let mut mouse_wheel_input_events =
                                world.resource_mut::<Events<MouseWheel>>();
                            mouse_wheel_input_events.send(MouseWheel {
                                unit: MouseScrollUnit::Pixel,
                                x,
                                y,
                            });
                        }
                    },
                };
            }
            baseview::Event::Keyboard(e) => {
                let mut keyboard_input_events = world.resource_mut::<Events<KeyboardInput>>();
                let key_code = keyboard::key_to_keycode(e.key);
                let state = match e.state {
                    keyboard_types::KeyState::Down => ButtonState::Pressed,
                    keyboard_types::KeyState::Up => ButtonState::Released,
                };
                let event = KeyboardInput {
                    scan_code: keyboard::code_to_scancode(e.code),
                    key_code,
                    state,
                };
                keyboard_input_events.send(event);
            }
            baseview::Event::Window(window_event) => {
                let mut windows = world.resource_mut::<Windows>();

                let window_id = if let Some(window_id) = get_window_id(&world, self.id()) {
                    window_id
                } else {
                    return status;
                };

                let window = if let Some(window) = windows.get_mut(window_id) {
                    window
                } else {
                    // If we're here, this window was previously opened
                    log::info!("Skipped event for closed window: {:?}", window_id);
                    return status;
                };
                match window_event {
                    baseview::WindowEvent::Resized(window_info) => {
                        // First adjust scale, if needed.
                        let scale_factor = window_info.scale();

                        if scale_factor != self.last_scale_factor {
                            let mut backend_scale_factor_change_events =
                                world.resource_mut::<Events<WindowBackendScaleFactorChanged>>();
                            backend_scale_factor_change_events.send(
                                WindowBackendScaleFactorChanged {
                                    id: window_id,
                                    scale_factor,
                                },
                            );

                            let mut scale_factor_change_events =
                                world.resource_mut::<Events<WindowScaleFactorChanged>>();

                            scale_factor_change_events.send(WindowScaleFactorChanged {
                                id: window_id,
                                scale_factor,
                            });

                            self.last_scale_factor = window_info.scale();
                            window.update_scale_factor_from_backend(self.last_scale_factor);
                        }

                        window.update_actual_size_from_backend(
                            window_info.physical_size().width,
                            window_info.physical_size().height,
                        );
                        let mut resize_events = world.resource_mut::<Events<WindowResized>>();
                        resize_events.send(WindowResized {
                            id: window_id,
                            width: window_info.logical_size().width as f32,
                            height: window_info.logical_size().height as f32,
                        });
                    }
                    baseview::WindowEvent::Focused => {
                        window.update_focused_status_from_backend(true);
                        let mut focused_events = world.resource_mut::<Events<WindowFocused>>();
                        focused_events.send(WindowFocused {
                            id: window_id,
                            focused: true,
                        });
                    }
                    baseview::WindowEvent::Unfocused => {
                        window.update_focused_status_from_backend(false);
                        let mut focused_events = world.resource_mut::<Events<WindowFocused>>();
                        focused_events.send(WindowFocused {
                            id: window_id,
                            focused: false,
                        });
                    }
                    baseview::WindowEvent::WillClose => {
                        status.shutdown = true;
                    }
                }
            }
        }
        status
    }
}

impl Default for BaseviewWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for BaseviewWindow {
    fn drop(&mut self) {
        log::info!("BaseviewWindow: drop");
    }
}

impl baseview::WindowHandler for BaseviewWindow {
    fn on_frame(&mut self, _window: &mut baseview::Window) {
        APP.with(|app_rc| {
            let mut app_ref_mut = app_rc.borrow_mut();
            match &mut *app_ref_mut {
                None => {}
                Some(app) => {
                    self.process_pending_events(app);
                    let windows = app.world.resource::<Windows>();
                    let focused = windows.iter().any(|w| w.is_focused());
                    handle_create_window_events(&mut app.world);
                    if focused {
                        app.update();
                    }
                }
            };
        });
    }

    fn on_event(
        &mut self,
        _window: &mut baseview::Window,
        event: baseview::Event,
    ) -> baseview::EventStatus {
        let gui_thread = GuiThread;

        let status = APP.with(|app_rc| {
            let mut app_ref_mut = app_rc.borrow_mut();

            match &mut *app_ref_mut {
                None => {
                    // If the App isn't yet available, store all window events,
                    // especially resize/scale factor related events.
                    self.pending_events.push_back(event);
                    // TODO: Determine whether this should be captured or ignored:
                    EventStatus {
                        return_status: baseview::EventStatus::Captured,
                        shutdown: false,
                    }
                }
                Some(app) => {
                    self.process_pending_events(app);
                    self.process_event(app, event)
                }
            }
        });

        if status.shutdown {
            drop_app(&gui_thread);
        }

        status.return_status
    }
}

fn change_window() {}

#[derive(Debug, Default)]
struct BaseviewCreateWindowReader(ManualEventReader<CreateWindow>);

fn handle_create_window_events(
    world: &mut World,
    //create_window_event_reader: &mut ManualEventReader<CreateWindow>,
) {
    let world = world.cell();
    let mut baseview_windows = world.non_send_resource_mut::<BaseviewWindows>();
    let mut windows = world.resource_mut::<Windows>();
    let create_window_events = world.resource::<Events<CreateWindow>>();
    let mut window_created_events = world.resource_mut::<Events<WindowCreated>>();
    #[cfg(not(any(target_os = "windows", target_feature = "x11")))]
    let mut window_resized_events = world.resource_mut::<Events<WindowResized>>();
    let mut create_window_event_reader = world
        .get_resource_mut::<BaseviewCreateWindowReader>()
        .expect("missing create window event reader");
    for create_window_event in create_window_event_reader.0.iter(&create_window_events) {
        log::info!(
            "bevy_baseview_plugin: handle_create_window_events: got event: {:?}",
            create_window_event
        );

        let baseview_window_info = world
            .get_non_send_resource::<BaseviewWindowInfo>()
            .expect("BaseviewWindowInfo is required;");

        let window = baseview_windows.create_window(
            create_window_event.id,
            &create_window_event.descriptor,
            baseview_window_info.clone(),
        );
        // This event is already sent on windows, x11, and xwayland.
        // TODO: we aren't yet sure about native wayland, so we might be able to exclude it,
        // but sending a duplicate event isn't problematic, as windows already does this.
        #[cfg(not(any(target_os = "windows", target_feature = "x11")))]
        window_resized_events.send(WindowResized {
            id: create_window_event.id,
            width: window.width(),
            height: window.height(),
        });
        windows.add(window);
        window_created_events.send(WindowCreated {
            id: create_window_event.id,
        });
    }
    //log::info!("baseview: handle_create_window_events: DONE",);
}
