use windows::core::HSTRING;
use windows::Data::Xml::Dom::*;
use windows::UI::Notifications::*;

pub fn show_simple() -> windows::core::Result<()> {
    let title = "featured picture of the day";
    let content = "beautiful scenery";
    let image = "https://picsum.photos/360/180?image=104";
    let logo = "https://picsum.photos/64?image=883";
    let toast_xml = format!(
        "<toast>
          <visual>
          <binding template='ToastGeneric'>
            <text>{}</text>
            <text>{}</text>
            <image src='{}'/>
            <image src='{}' placement='appLogoOverride' hint-crop='circle'/>
          </binding>
        </visual>
      </toast>",
        title, content, image, logo
    );
    let xml = XmlDocument::new()?;
    xml.LoadXml(&HSTRING::from(toast_xml))?;
    let notification = ToastNotification::CreateToastNotification(&xml)?;
    let notifier = ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from("wando-rs"))?;
    return notifier.Show(&notification);
}
