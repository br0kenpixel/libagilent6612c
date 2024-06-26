use agilent6612c::{device::Agilent6612c, params::ConnectionParameters};

fn main() {
    let params = ConnectionParameters::default();
    let mut device = Agilent6612c::new("/dev/cu.usbserial-14130", params, None).unwrap();

    println!("Connected");

    let hw = device.hwinfo().unwrap();
    println!("{hw}");
}
