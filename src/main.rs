use log::{debug, info};

mod app;
mod window;

fn main() -> anyhow::Result<()> {
    // initialize the logger
    env_logger::init();
    debug!("Logger has loaded");

    // --- put stuff to do before app is launched ---

    info!("Launching the app.");
    app::run()?;

    // --- put stuff to do before program is exited ---

    info!("Exiting...");
    Ok(())
}
