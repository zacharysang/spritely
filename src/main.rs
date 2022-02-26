use std::time::SystemTime;
use std::fmt;

use log::{info, trace, warn, error, LevelFilter};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use crate::world::World;

// root the world module at the root of the crate?
mod world;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: i16 = 8;

fn init_logging() {
    let startTime = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => format!("{}", n.as_secs()),
        Err(_) => format!("unknown-time")
    };
    let logFileName = format!("log-{}.log", startTime);
    simple_logging::log_to_file(&logFileName, LevelFilter::Info);

    info!("Opened log with filename: {}", logFileName);
}

fn main() -> Result<(), Error> {

    init_logging();

    // below is all from https://github.com/parasyte/pixels/blob/94a2cc2dbdba493dcbec1e99c226a06a23088319/examples/minimal-winit/src/main.rs
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);

        // lack of semicolon means that this block evaluates to the value of the belowv (and this value is assigned to 'window')
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut world = World::new(WIDTH, HEIGHT, BOX_SIZE);

    event_loop.run(move |event, _, control_flow| {

        // If the event passed into this loop iteration matches the RedrawRequested type, then redraw
        if let Event::RedrawRequested(_) = event {

            // pass in the mutable pixel frame buffer to be written to for the newly requested rendering
            world.draw(pixels.get_frame());

            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}