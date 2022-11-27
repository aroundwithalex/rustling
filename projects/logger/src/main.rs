#[macro_use]
extern crate log;

fn main() {
    env_logger::init();

    info!("Info Message");
    warn!("Warn Message");
    error!("Error Message");
    debug!("Debug Message");

    // Run with RUST_LOG=debug cargo new
}
