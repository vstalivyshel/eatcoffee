mod setup;
mod init;
use setup::*;
use init::start;
pub use init::{ Spawner, Example};

pub fn draw<E: Example>(title: &str) {
    let setup = pollster::block_on(setup::<E>(title));
    start::<E>(setup);
}
