mod points;
mod state;
mod pointcloud;
mod app;

use app::App;
use winit::event_loop::EventLoop;
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

    let mut app = App::new();

    #[cfg(target_arch = "wasm32")]{
        use winit::platform::web::EventLoopExtWebSys;
        app.make_state(&event_loop).await;
        let _ = event_loop.spawn_app(app);    
    }

    #[cfg(not(target_arch = "wasm32"))]
    let _ = event_loop.run_app(&mut app);
}
