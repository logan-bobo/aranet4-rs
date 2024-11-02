use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

const BATTERY_UUID: &str = "00002a19-0000-1000-8000-00805f9b34fb";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    let adapters = manager.adapters().await?;

    let central = adapters.into_iter().nth(0).unwrap();
    central.start_scan(ScanFilter::default()).await?;
    time::sleep(Duration::from_secs(5)).await;

    let device = find_aranet4(&central).await.unwrap();
    device.connect().await?;

    device.discover_services().await?;

    // find the characteristic we want
    let chars = device.characteristics();

    println!("{chars:#?}");

    let uuid_to_test: Uuid = Uuid::from_str(BATTERY_UUID).expect("To handle");

    let cmd_char = chars.iter().find(|c| c.uuid == uuid_to_test).unwrap();

    let battery_level = device.read(&cmd_char).await?;

    let battery_level = battery_level[0];

    println!("Device Battery Level: {battery_level:#?}%");

    Ok(())
}

async fn find_aranet4(central: &Adapter) -> Option<Peripheral> {
    for p in central.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("Aranet"))
        {
            println!("Found Aranet");
            return Some(p);
        }
    }
    None
}
