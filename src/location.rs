use windows::core::Result;
use windows::Devices::Geolocation::{Geolocator, Geoposition, PositionAccuracy};

pub async fn get_location() -> Result<Geoposition> {
    let geolocator = Geolocator::new()?;

    geolocator.SetDesiredAccuracy(PositionAccuracy::High)?;
    let position = geolocator.GetGeopositionAsync()?.await?;
    let info = position.Coordinate()?.Point()?.Position()?;

    println!("Latitude: {}", info.Latitude);
    println!("Longitude: {}", info.Longitude);

    Ok(position)
}
