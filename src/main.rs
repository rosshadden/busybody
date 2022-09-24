use rust_gpiozero::*;

fn main() {
	let mut led = LED::new(8);

	led.blink(2.0, 3.0);

	led.wait();
}
