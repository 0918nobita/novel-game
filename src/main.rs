extern crate game;
#[macro_use]
extern crate log;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let entry = unsafe { ash::Entry::new() }.context("Failed to create ash::Entry")?;
    let instance = game::ManagedInstance::new(&entry)?;
    let physical_devices = instance.enumerate_physical_device()?;
    for physical_device in physical_devices.iter() {
        debug!("Physical Device: {:?}", *physical_device)
    }
    debug!("Complete!");
    Ok(())
}
