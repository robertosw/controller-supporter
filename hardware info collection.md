# Source Code
Run without root access
```Rust
let devices = evdev::enumerate().map(|tuple| tuple.1);
for device in devices {
    println!("device  - {:?}", device.toString());
}
```

<br>

# PS5 Controller
```YAML
Wireless Controller:
  Driver version: 1.0.1
  Unique name: "88:03:4c:f5:7b:99"
  Bus: Bluetooth
  Vendor: 0x54c
  Product: 0xce6
  Version: 0x8100
  Properties: {}
  Keys supported:
    BTN_SOUTH (index 304)
    BTN_EAST (index 305)
    BTN_NORTH (index 307)
    BTN_WEST (index 308)
    BTN_TL (index 310)
    BTN_TR (index 311)
    BTN_TL2 (index 312)
    BTN_TR2 (index 313)
    BTN_SELECT (index 314)
    BTN_START (index 315)
    BTN_MODE (index 316)
    BTN_THUMBL (index 317)
    BTN_THUMBR (index 318)
  Absolute Axes:
    ABS_X (input_absinfo { value: 0, minimum: 0, maximum: 0, fuzz: 0, flat: 0, resolution: 0 }, index 0)
    ABS_Y (input_absinfo { value: 0, minimum: 0, maximum: 0, fuzz: 0, flat: 0, resolution: 0 }, index 1)
    ABS_Z (input_absinfo { value: 0, minimum: 0, maximum: 0, fuzz: 0, flat: 0, resolution: 0 }, index 2)
    ABS_RX (input_absinfo { value: 0, minimum: 0, maximum: 0, fuzz: 0, flat: 0, resolution: 0 }, index 3)
    ABS_RY (input_absinfo { value: 0, minimum: 0, maximum: 0, fuzz: 0, flat: 0, resolution: 0 }, index 4)
    ABS_RZ (input_absinfo { value: 0, minimum: 0, maximum: 0, fuzz: 0, flat: 0, resolution: 0 }, index 5)
    ABS_HAT0X (input_absinfo { value: 0, minimum: 0, maximum: 0, fuzz: 0, flat: 0, resolution: 0 }, index 16)
    ABS_HAT0Y (input_absinfo { value: 0, minimum: 0, maximum: 0, fuzz: 0, flat: 0, resolution: 0 }, index 17)
  Force Feedback supported
```
