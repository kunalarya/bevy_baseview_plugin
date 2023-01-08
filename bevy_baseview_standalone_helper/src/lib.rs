//! App harness allowing users to create standalone bevy apps to develop the GUI independent of the
//! baseview layer.

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use raw_window_handle::HasRawWindowHandle;
use raw_window_handle::RawWindowHandle;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use bevy_baseview_plugin::{AppProxy, ParentWin};

// Window size (logical).
const WINDOW_WIDTH: f64 = 500.0;
const WINDOW_HEIGHT: f64 = 400.0;

struct AppWrapper<F: Fn(ParentWin, f64, f64) -> AppProxy> {
    initialized: AtomicBool,
    parent_win: ParentWin,
    create_app: F,
    app: Option<AppProxy>,
}

impl<F: Fn(ParentWin, f64, f64) -> AppProxy> AppWrapper<F> {
    pub fn new(raw_window_handle: RawWindowHandle, create_app: F) -> Self {
        let parent_win = ParentWin::from(raw_window_handle);
        let initialized = AtomicBool::new(false);
        Self {
            initialized,
            parent_win,
            create_app,
            app: None,
        }
    }
    pub fn receive_events<'a>(
        &mut self,
        window: &winit::window::Window,
        event: Event<'a, ()>,
        control_flow: &mut ControlFlow,
    ) {
        *control_flow = ControlFlow::Poll;

        if let Ok(_) =
            self.initialized
                .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        {
            self.app = Some((self.create_app)(
                self.parent_win.clone(),
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
            ));
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    }
}

pub fn run_app<F: Fn(ParentWin, f64, f64) -> AppProxy + 'static>(create_app: F) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build(&event_loop)
        .unwrap();

    let mut app_wrapper = AppWrapper::new(window.raw_window_handle(), create_app);
    event_loop.run(move |event, _, control_flow| {
        app_wrapper.receive_events(&window, event, control_flow)
    });
}
