use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED1: u8 = 23;
const GPIO_LED2: u8 = 24;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut pin1 = Gpio::new()?.get(GPIO_LED1)?.into_output();
    let mut pin2 = Gpio::new()?.get(GPIO_LED2)?.into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    pin1.set_high();
    thread::sleep(Duration::from_millis(500));
    pin1.set_low();

    pin2.set_high();
    thread::sleep(Duration::from_millis(500));
    pin2.set_low();

    Ok(())
}
