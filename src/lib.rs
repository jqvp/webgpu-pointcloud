mod points;
mod state;
mod pointcloud;

use std::sync::Arc;
use winit::{
    event::WindowEvent,
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

use state::*;

#[allow(unused)]
const WIDTH: u32 = 500;
#[allow(unused)]
const HEIGHT: u32 = 500;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new().unwrap();

    // State::new uses async code, so we're going to wait for it to finish
    let mut window_state = StateApplication::new();
    let _ = event_loop.run_app(&mut window_state);
}


struct StateApplication {
    state: Option<State>,
}

impl StateApplication {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl ApplicationHandler for StateApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop
                    .create_window(Window::default_attributes()
                    .with_title("Pointcloud Viewer"))
                    .unwrap());

        #[cfg(target_arch = "wasm32")] {
            use winit::dpi::PhysicalSize;
            use winit::platform::web::WindowExtWebSys;

            let _ = window.request_inner_size(PhysicalSize::new(WIDTH, HEIGHT));

            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("body")?;
                    let canvas = window.canvas()?;
                    canvas.set_height(HEIGHT);
                    canvas.set_width(WIDTH);
                    let canvas2 = web_sys::Element::from(canvas);
                    dst.append_child(&canvas2).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        self.state = Some(State::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = self.state.as_ref().unwrap().window();

        if window.id() == window_id {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    self.state.as_mut().unwrap().resize(physical_size);
                }
                WindowEvent::RedrawRequested => {
                    self.state.as_mut().unwrap().update();
                    self.state.as_mut().unwrap().render().expect("Render ERROR!");
                }
                _ => {}
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let window = self.state.as_ref().unwrap().window();
        window.request_redraw();
    }
}
