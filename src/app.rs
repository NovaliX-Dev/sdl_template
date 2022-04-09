use log::{debug, info};
use sdl2::{
    event::Event as SDLEvent,
    pixels::Color
};

use crate::window;

/// Launch the app
pub fn run() -> anyhow::Result<()> {
    debug!("Initialize SDL modules...");
    let (video, mut event_pump) = window::init_sdl_modules()
        .map_err(|e| anyhow::anyhow!(e))?;

    info!("Creating the window...");
    let mut canvas = window::create_window(
        &video, 
        "Window", 
        800, 
        600, 
        true, 
        true
    )
        .map_err(|e| anyhow::anyhow!(e))?;

    info!("Entering window loop...");

    'win_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                // window close event (if there is only one)
                SDLEvent::Quit { .. } => {
                    info!("Window closed");

                    break 'win_loop
                },

                // --- Add others event here ---

                _ => ()
            }
        }

        // --- Operations put here will be called every frame ---

        // clear the canvas
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // --- Put the rendering stuff here ---

        // update the window
        canvas.present();
    }

    Ok(())
}