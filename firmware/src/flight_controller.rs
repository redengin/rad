use crate::{time::{Duration, Timer}, Vehicle};

#[embassy_executor::task]
pub async fn thread(vehicle:Vehicle<'static>)
{
    loop
    {
        log::debug!("flight_controller working....");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}
