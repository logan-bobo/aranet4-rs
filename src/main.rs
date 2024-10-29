use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::time::Duration;
use tokio::time;

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
