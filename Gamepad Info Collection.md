# Raspi Rust Target

Installing rust with rustup provides that the Raspberry Pi Zero 2 is `aarch64-unknown-linux-gnu`

<br>

# Variant 1 - Rust & rusb
Run without root access
```Rust
let devices = evdev::enumerate().map(|tuple| tuple.1);
for device in devices {
    println!("device  - {:?}", device.toString());
}
```

<br>

### PS5 Gamepad via USB
```YAML
Wireless Controller:
  Driver version: 1.0.1
  Unique name: "mac:address:here"
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

<br>

# Variant 2 - Connection establishment log
[source](https://www.slashdev.ca/2010/05/08/get-usb-report-descriptor-with-linux/)

1. `sudo dmesg -C` -> clear Buffer
2. connect gamepad via usb
3. `sudo dmesg -c`

### PS5 Gamepad via USB
```Shell
[ 6743.550282] usb 1-2: new high-speed USB device number 9 using xhci_hcd
[ 6743.701214] usb 1-2: New USB device found, idVendor=054c, idProduct=0ce6, bcdDevice= 1.00
[ 6743.701225] usb 1-2: New USB device strings: Mfr=1, Product=2, SerialNumber=0
[ 6743.701229] usb 1-2: Product: Wireless Controller
[ 6743.701232] usb 1-2: Manufacturer: Sony Interactive Entertainment
[ 6743.928192] playstation 0003:054C:0CE6.000A: hidraw4: USB HID v1.11 Gamepad [Sony Interactive Entertainment Wireless Controller] on usb-0000:03:00.3-2/input3
[ 6743.991507] input: Sony Interactive Entertainment Wireless Controller as /devices/pci0000:00/0000:00:08.1/0000:03:00.3/usb1/1-2/1-2:1.3/0003:054C:0CE6.000A/input/input39
[ 6743.991739] input: Sony Interactive Entertainment Wireless Controller Motion Sensors as /devices/pci0000:00/0000:00:08.1/0000:03:00.3/usb1/1-2/1-2:1.3/0003:054C:0CE6.000A/input/input40
[ 6743.991866] input: Sony Interactive Entertainment Wireless Controller Touchpad as /devices/pci0000:00/0000:00:08.1/0000:03:00.3/usb1/1-2/1-2:1.3/0003:054C:0CE6.000A/input/input41
[ 6743.994024] playstation 0003:054C:0CE6.000A: Registered DualSense controller hw_version=0x00000514 fw_version=0x01040027
```

<br>

# Variant 3 - lsusb
[source](https://www.slashdev.ca/2010/05/08/get-usb-report-descriptor-with-linux/)

1. `lsusb` -> Find your Controller in List and Copy `vendor:product`
2. `lsusb -vd vendor:product`

### PS5 Gamepad via USB
```YAML
Bus 001 Device 007: ID 054c:0ce6 Sony Corp. Wireless Controller
Device Descriptor:
  bLength                18
  bDescriptorType         1
  bcdUSB               2.00
  bDeviceClass            0 
  bDeviceSubClass         0 
  bDeviceProtocol         0 
  bMaxPacketSize0        64
  idVendor           0x054c Sony Corp.
  idProduct          0x0ce6 
  bcdDevice            1.00
  iManufacturer           1 Sony Interactive Entertainment
  iProduct                2 Wireless Controller
  iSerial                 0 
  bNumConfigurations      1
  Configuration Descriptor:
    bLength                 9
    bDescriptorType         2
    wTotalLength       0x00e3
    bNumInterfaces          4
    bConfigurationValue     1
    iConfiguration          0 
    bmAttributes         0xc0
      Self Powered
    MaxPower              500mA
    Interface Descriptor:
    ... audio interfaces
  
    Interface Descriptor:
      bLength                 9
      bDescriptorType         4
      bInterfaceNumber        3
      bAlternateSetting       0
      bNumEndpoints           2
      bInterfaceClass         3 Human Interface Device
      bInterfaceSubClass      0 
      bInterfaceProtocol      0 
      iInterface              0 
        HID Device Descriptor:
          bLength                 9
          bDescriptorType        33
          bcdHID               1.11
          bCountryCode            0 Not supported
          bNumDescriptors         1
          bDescriptorType        34 Report
          wDescriptorLength     273
         Report Descriptors: 
           ** UNAVAILABLE **
      Endpoint Descriptor:
        bLength                 7
        bDescriptorType         5
        bEndpointAddress     0x84  EP 4 IN
        bmAttributes            3
          Transfer Type            Interrupt
          Synch Type               None
          Usage Type               Data
        wMaxPacketSize     0x0040  1x 64 bytes
        bInterval               6
      Endpoint Descriptor:
        bLength                 7
        bDescriptorType         5
        bEndpointAddress     0x03  EP 3 OUT
        bmAttributes            3
          Transfer Type            Interrupt
          Synch Type               None
          Usage Type               Data
        wMaxPacketSize     0x0040  1x 64 bytes
        bInterval               6
Device Qualifier (for other device speed):
  bLength                10
  bDescriptorType         6
  bcdUSB               2.00
  bDeviceClass            0 
  bDeviceSubClass         0 
  bDeviceProtocol         0 
  bMaxPacketSize0        64
  bNumConfigurations      1
can't get debug descriptor: Resource temporarily unavailable
Device Status:     0x0000
  (Bus Powered)

```

<br>

# Variant 4 - usbhid-dump
[git repo](https://github.com/DIGImend/usbhid-dump)

1. Download .tar from git repo and build local
2. `lsusb` copy `vendor` and `product`
3. `usbhid-dump --model=vendorid:productid`

### PS5 Gamepad via usb
```Shell
usbhid-dump --model=054c:0ce6
001:010:003:DESCRIPTOR         1692698824.638454
 05 01 09 05 A1 01 85 01 09 30 09 31 09 32 09 35
 09 33 09 34 15 00 26 FF 00 75 08 95 06 81 02 06
 00 FF 09 20 95 01 81 02 05 01 09 39 15 00 25 07
 35 00 46 3B 01 65 14 75 04 95 01 81 42 65 00 05
 09 19 01 29 0F 15 00 25 01 75 01 95 0F 81 02 06
 00 FF 09 21 95 0D 81 02 06 00 FF 09 22 15 00 26
 FF 00 75 08 95 34 81 02 85 02 09 23 95 2F 91 02
 85 05 09 33 95 28 B1 02 85 08 09 34 95 2F B1 02
 85 09 09 24 95 13 B1 02 85 0A 09 25 95 1A B1 02
 85 20 09 26 95 3F B1 02 85 21 09 27 95 04 B1 02
 85 22 09 40 95 3F B1 02 85 80 09 28 95 3F B1 02
 85 81 09 29 95 3F B1 02 85 82 09 2A 95 09 B1 02
 85 83 09 2B 95 3F B1 02 85 84 09 2C 95 3F B1 02
 85 85 09 2D 95 02 B1 02 85 A0 09 2E 95 01 B1 02
 85 E0 09 2F 95 3F B1 02 85 F0 09 30 95 3F B1 02
 85 F1 09 31 95 3F B1 02 85 F2 09 32 95 0F B1 02
 85 F4 09 35 95 3F B1 02 85 F5 09 36 95 03 B1 02
 C0
```


