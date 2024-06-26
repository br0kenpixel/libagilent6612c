use agilent6612c::{device::Agilent6612c, params::ConnectionParameters};

fn main() {
    let params = ConnectionParameters::default();
    let mut device = Agilent6612c::new("/dev/cu.usbserial-14130", params, None).unwrap();

    println!("Connected");

    let hw = device.hwinfo().unwrap();
    let ver = device.firmware_version().unwrap();
    println!("{hw}, FW ver: {ver}");

    let maxv = device.maximum_voltage().unwrap();
    let maxi = device.maximum_current().unwrap();

    println!("Maximum values: {maxv:.0}V, {maxi:.0}A");
    println!("OCP: {}", device.ocp().unwrap());
}
