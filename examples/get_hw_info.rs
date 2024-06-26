use agilent6612c::{device::Agilent6612c, params::ConnectionParametersBuilder};

fn main() {
    let params = ConnectionParametersBuilder::default().build().unwrap();
    let mut device = Agilent6612c::new("/dev/", params, None).unwrap();

    println!("Connected");

    let hw = device.hwinfo().unwrap();
    println!("{hw}");
}
