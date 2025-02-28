use crate::settings::Setting;
use anyhow::Context;
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

mod management;
mod settings;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Setting::emerge().context("reading config")?;

    management::run(settings.config.management_port).await?;

    Ok(())
}
