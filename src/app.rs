use std::sync::Arc;
use cfg_if::cfg_if;
use winit::{
    event::WindowEvent,
    application::ApplicationHandler,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};
#[cfg(not(target_arch = "wasm32"))]
use pollster::FutureExt;
#[cfg(target_arch = "wasm32")]
use futures::channel::oneshot::Receiver;

use crate::state::State;

#[allow(unused)]
const WIDTH: u32 = 500;
#[allow(unused)]
const HEIGHT: u32 = 500;

pub struct StateApplication {
    state: Option<State>,
    #[cfg(target_arch = "wasm32")]
    startup_receiver: Option<Receiver<State>>
}

impl StateApplication {
    pub fn new() -> Self {
        Self { 
            state: None,
            #[cfg(target_arch = "wasm32")]
            startup_receiver: None
        }
    }
}

impl ApplicationHandler for StateApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut attributes = Window::default_attributes()
            .with_title("Pointcloud Viewer");

        #[cfg(target_arch = "wasm32")] {
            use winit::platform::web::WindowAttributesExtWebSys;
            use wasm_bindgen::JsCast;

            let canvas = wgpu::web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("canvas")
                .unwrap()
                .dyn_into::<wgpu::web_sys::HtmlCanvasElement>()
                .unwrap();
            canvas.set_height(HEIGHT);
            canvas.set_width(WIDTH);
            attributes = attributes.with_canvas(Some(canvas));
        }
        
        let window = Arc::new(event_loop
            .create_window(attributes)
            .expect("Couldn't put title")
        );

        cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                use winit::dpi::PhysicalSize;
                let _ = window.request_inner_size(PhysicalSize::new(WIDTH, HEIGHT));

                let (sender, receiver) = futures::channel::oneshot::channel();
                    self.startup_receiver = Some(receiver);
                    wasm_bindgen_futures::spawn_local(async move {
                        let state = State::new(window).await;
                        if sender.send(state).is_err() {
                            log::error!("Failed to create and send renderer!");
                        }
                    });
            } else {
                self.state = Some(State::new(window).block_on());
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        #[cfg(target_arch = "wasm32")] {  
            if let Some(receiver) = self.startup_receiver.as_mut() {
                if let Ok(Some(state)) = receiver.try_recv() {
                    self.state = Some(state);
                    self.startup_receiver = None;
                }
            }
        }

        let Some(state) = self.state.as_mut() else {
            return;
        };

        let window = state.window();
        window.request_redraw();

        if window.id() == window_id {
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
                }
                _ => {}
            }
        }
    }


}
