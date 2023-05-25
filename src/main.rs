mod location;
mod media_player;
mod notifications;

use futures::executor::block_on;

async fn async_main() -> windows::core::Result<()> {
    //    media_player::play().await?;
    // requires app registration so the notifications actually show
    //    notifications::show_simple()?;
    location::get_location().await?;
    Ok(())
}

fn main() -> windows::core::Result<()> {
    block_on(async_main())?;
    Ok(())
}
