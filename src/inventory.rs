use windows::core::Result;
use windows::System::Inventory::InstalledDesktopApp;

pub async fn get_installed_apps() -> Result<()> {
    let apps = InstalledDesktopApp::GetInventoryAsync()?.await?;
    for app in apps {
        println!("App: {}", app.DisplayName()?);
    }
    Ok(())
}
