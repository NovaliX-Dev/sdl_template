use log::trace;
use sdl2::{
    VideoSubsystem, 
    EventPump, 
    render::Canvas, 
    video::Window
};

/// Init all the needed SDL modules
pub fn init_sdl_modules() -> Result<(VideoSubsystem, EventPump), String> {
    let sdl_context = sdl2::init()?;

    let video = sdl_context.video()?;
    let event_pump = sdl_context.event_pump()?;

    // --- initialize others modules here ---

    Ok((video, event_pump))
}

/// Create a SDL window
pub fn create_window(
    video: &VideoSubsystem, 
    title: &str, 
    width: u32, 
    height: u32, 
    resizable: bool, 
    accelerated: bool
) -> Result<Canvas<Window>, String> {
    trace!("Creating and configuring the window...");

    // configure the window builder
    let mut w_builder = video.window(title, width, height);
    if resizable {
        w_builder.resizable();
    }

    // --- Add others window configuration here ---

    let window = w_builder.build()
        .map_err(|e| e.to_string())?;

    trace!("Creating and configuring the canvas...");

    // configure the canvas builder
    let mut c_builder = window.into_canvas();
    if accelerated {
        c_builder = c_builder.accelerated()
    };

    // --- Add others canvas configuration here ---

    let canvas = c_builder.build()
        .map_err(|e| e.to_string())?;

    Ok(canvas)
}