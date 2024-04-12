use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

const GPIO_LED1: u8 = 23; // pin 16
const GPIO_LED2: u8 = 24; // pin 18
const GPIO_LED3: u8 = 22; // pin 18

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut pin1 = Gpio::new()?.get(GPIO_LED1)?.into_output();
    let mut pin2 = Gpio::new()?.get(GPIO_LED2)?.into_output();
    let mut pin2 = Gpio::new()?.get(GPIO_LED3)?.into_output();

    loop {
        pin1.set_high();
        thread::sleep(Duration::from_millis(500));
        pin1.set_low();

        thread::sleep(Duration::from_millis(500));

        pin2.set_high();
        thread::sleep(Duration::from_millis(500));
        pin2.set_low();

        thread::sleep(Duration::from_millis(500));

        pin3.set_high();
        thread::sleep(Duration::from_millis(500));
        pin3.set_low();

        thread::sleep(Duration::from_millis(500));
    }


    Ok(())
}
