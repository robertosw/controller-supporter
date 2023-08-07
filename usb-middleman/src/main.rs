#![allow(unused_imports, dead_code)]

mod hidapi;
mod rusb;
use crate::{rusb::*, hidapi::*};

fn main() {
    hidapi();
    // _rusb();
}
