use std::sync::Arc;
use winit::{
    event::WindowEvent,
    application::ApplicationHandler,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};


use crate::state::*;

pub struct StateApplication {
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
                .unwrap()
            );

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
