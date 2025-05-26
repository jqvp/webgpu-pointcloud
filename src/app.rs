use std::sync::Arc;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}
};
#[cfg(not(target_family = "wasm"))]
use pollster::FutureExt;
#[cfg(target_family = "wasm")]
use winit::event_loop::EventLoop;

use crate::engine::Engine;

#[allow(unused)]
const WIDTH: u32 = 500;
#[allow(unused)]
const HEIGHT: u32 = 500;

pub struct App {
    state: Option<Engine>,
}

impl App {
    pub const fn new() -> Self {
        Self { 
            state: None,
        }
    }
}

impl App {
    pub async fn make_state(
        &mut self,
        #[cfg(target_family = "wasm")]
        event_loop: &EventLoop<()>,
        #[cfg(not(target_family = "wasm"))]
        event_loop: &ActiveEventLoop,
    ) {
        let attributes = Window::default_attributes()
            .with_title("Pointcloud Viewer");

        let window = event_loop
            .create_window(attributes)
            .expect("Couldn't create window");

        #[cfg(target_arch = "wasm32")] {
            use winit::dpi::PhysicalSize;
            let _ = window.request_inner_size(PhysicalSize::new(WIDTH, HEIGHT));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("body")?;
                    let canvas = web_sys::HtmlCanvasElement::from(window.canvas()?);
                    canvas.set_height(HEIGHT);
                    canvas.set_width(WIDTH);
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        self.state = Some((Engine::new(Arc::new(window))).await);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[cfg(not(target_family = "wasm"))]
        if self.state.is_none() { // Assume there is no window if we haven't made a State already
            self.make_state(event_loop).block_on();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = self.state.as_mut().unwrap();

        state.window().request_redraw();

        if state.window().id() == window_id {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    state.resize(physical_size);
                }
                WindowEvent::RedrawRequested => {
                    state.update();
                    state.render().expect("Render ERROR!");
                },
                WindowEvent::KeyboardInput { event, ..} => {
                    state.input(&event);
                },
                _ => {}
            }
        }
    }


}
