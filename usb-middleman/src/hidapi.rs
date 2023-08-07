use hidapi::HidApi;

pub fn hidapi() {
    let api = HidApi::new().unwrap();

    let mut vid: u16 = 0;
    let mut pid: u16 = 0;

    // Print out information about all connected devices
    for device in api.device_list() {
        if device.vendor_id() == 1356 {
            println!("{:#?}", device);
            vid = 1356;
            pid = device.product_id();
        }
    }

    let device = match api.open(vid, pid) {
        Ok(hid_device) => hid_device,
        Err(err) => panic!("Error: {:?}", err),
    };

    // Read data from device
    let mut buf = [0 as u8; 8];
    let res = device.read(&mut buf[..]).unwrap();
    println!("Read: {:?}", &buf[..res]);
}
