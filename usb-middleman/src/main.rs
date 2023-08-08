#![allow(unused_imports, dead_code)]

mod hidapi;
mod rusb;
mod hidapi_structs;
use crate::{hidapi::*, rusb::*};

fn main() {
    hidapi();
    // _rusb();
}

// Terminal 1: .../controller-supporter/usb-middleman$                  clear && cargo build --release
// Terminal 2: .../controller-supporter/usb-middleman/target/release$   clear && sudo chown root usb-middleman && sudo chmod u+s usb-middleman && ./usb-middleman
