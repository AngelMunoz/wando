mod data_protection;
mod inventory;
mod location;
mod media_player;
mod notifications;

use futures::executor::block_on;
use windows::core::Result;

async fn async_main() -> Result<()> {
    //    media_player::play().await?;

    // requires app registration so the notifications actually show
    //    notifications::show_simple()?;

    // location::get_location().await?;

    // inventory::get_installed_apps().await?;

    data_protection::run_protection().await?;
    Ok(())
}

fn main() -> Result<()> {
    block_on(async_main())?;
    Ok(())
}
