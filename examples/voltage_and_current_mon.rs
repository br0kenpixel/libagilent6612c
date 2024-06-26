use std::{thread::sleep, time::Duration};

use agilent6612c::{device::Agilent6612c, params::ConnectionParameters};

fn main() {
    let params = ConnectionParameters::default();
    let mut device = Agilent6612c::new("/dev/cu.usbserial-14130", params, None).unwrap();

    println!("Connected");

    println!("Setting safe voltage and current: 1V, 10mA");
    device.set_output_voltage(1.0).unwrap();
    device.set_output_current(0.010).unwrap();

    println!("Enabling OCP");
    device.set_ocp(true).unwrap();

    println!("Press CTRL-C to stop");
    loop {
        let voltage = device.measure_voltage().unwrap();
        let current = device.measure_current().unwrap();

        println!("{voltage:.2}V - {current:.2}A");
        sleep(Duration::from_secs(1));
    }
}
