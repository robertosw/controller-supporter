- Installing rust with rustup provides that the Raspberry Pi Zero 2 is `aarch64-unknown-linux-gnu`
- `/sys/bus/hid/devices/` hid devices here probably have the same folder structure as is needed for by the gadget driver
- [HID Report Descriptors Syntax Intro](https://www.kernel.org/doc/html/next/hid/hidintro.html)
- `lspci -v|grep HCI` find out which type of **Host Controller Driver** you have on your hardware
- `hid-recorder` from the `hid-tools` package does alot of this in one step

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
### Comparison actual PS5 Gamepad (-) and mirrored PS5 Gamepad (+)
```diff
Device Descriptor:
-  iSerial                 0 
+  iSerial                 3                    // Might be fixable by not writing any string to serialnumber
  Configuration Descriptor:
-    wTotalLength       0x00e3
-    bNumInterfaces          4
-    iConfiguration          0 
+    wTotalLength       0x0029
+    bNumInterfaces          1
+    iConfiguration          4                  // Might be fixable by not writing any string to configs
    Interface Descriptor:
-      bInterfaceNumber        3
-      iInterface              0
+      bInterfaceNumber        0
+      iInterface              5 HID Interface
        HID Device Descriptor:
-          bcdHID               1.11
+          bcdHID               1.01
      Endpoint Descriptor:
-        bEndpointAddress     0x84  EP 4 IN     
-        bInterval               6
+        bEndpointAddress     0x81  EP 1 IN     // These might change automatically if you add audio Interfaces
+        bInterval               4
      Endpoint Descriptor:
-        bEndpointAddress     0x03  EP 3 OUT
-        bInterval               6
+        bEndpointAddress     0x01  EP 1 OUT    // These might change automatically if you add audio Interfaces
+        bInterval               4        
```

### PS4 Gamepad via USB
```YAML
Bus 001 Device 007: ID 054c:09cc Sony Corp. DualShock 4 [CUH-ZCT2x]
Device Descriptor:
  bLength                18
  bDescriptorType         1
  bcdUSB               2.00
  bDeviceClass            0 
  bDeviceSubClass         0 
  bDeviceProtocol         0 
  bMaxPacketSize0        64
  idVendor           0x054c Sony Corp.
  idProduct          0x09cc DualShock 4 [CUH-ZCT2x]
  bcdDevice            1.00
  iManufacturer           1 Sony Interactive Entertainment
  iProduct                2 Wireless Controller
  iSerial                 0 
  bNumConfigurations      1
  Configuration Descriptor:
    bLength                 9
    bDescriptorType         2
    wTotalLength       0x00e1
    bNumInterfaces          4
    bConfigurationValue     1
    iConfiguration          0 
    bmAttributes         0xc0
      Self Powered
    MaxPower              500mA
    Interface Descriptor:
      bLength                 9
      bDescriptorType         4
      bInterfaceNumber        0
      bAlternateSetting       0
      bNumEndpoints           0
      bInterfaceClass         1 Audio
      bInterfaceSubClass      1 Control Device
      bInterfaceProtocol      0 
      iInterface              0 
      AudioControl Interface Descriptor:
        bLength                10
        bDescriptorType        36
        bDescriptorSubtype      1 (HEADER)
        bcdADC               1.00
        wTotalLength       0x0047
        bInCollection           2
        baInterfaceNr(0)        1
        baInterfaceNr(1)        2
      AudioControl Interface Descriptor:
        bLength                12
        bDescriptorType        36
        bDescriptorSubtype      2 (INPUT_TERMINAL)
        bTerminalID             1
        wTerminalType      0x0101 USB Streaming
        bAssocTerminal          6
        bNrChannels             2
        wChannelConfig     0x0003
          Left Front (L)
          Right Front (R)
        iChannelNames           0 
        iTerminal               0 
      AudioControl Interface Descriptor:
        bLength                10
        bDescriptorType        36
        bDescriptorSubtype      6 (FEATURE_UNIT)
        bUnitID                 2
        bSourceID               1
        bControlSize            1
        bmaControls(0)       0x03
          Mute Control
          Volume Control
        bmaControls(1)       0x00
        bmaControls(2)       0x00
        iFeature                0 
      AudioControl Interface Descriptor:
        bLength                 9
        bDescriptorType        36
        bDescriptorSubtype      3 (OUTPUT_TERMINAL)
        bTerminalID             3
        wTerminalType      0x0402 Headset
        bAssocTerminal          4
        bSourceID               2
        iTerminal               0 
      AudioControl Interface Descriptor:
        bLength                12
        bDescriptorType        36
        bDescriptorSubtype      2 (INPUT_TERMINAL)
        bTerminalID             4
        wTerminalType      0x0402 Headset
        bAssocTerminal          3
        bNrChannels             1
        wChannelConfig     0x0000
        iChannelNames           0 
        iTerminal               0 
      AudioControl Interface Descriptor:
        bLength                 9
        bDescriptorType        36
        bDescriptorSubtype      6 (FEATURE_UNIT)
        bUnitID                 5
        bSourceID               4
        bControlSize            1
        bmaControls(0)       0x03
          Mute Control
          Volume Control
        bmaControls(1)       0x00
        iFeature                0 
      AudioControl Interface Descriptor:
        bLength                 9
        bDescriptorType        36
        bDescriptorSubtype      3 (OUTPUT_TERMINAL)
        bTerminalID             6
        wTerminalType      0x0101 USB Streaming
        bAssocTerminal          1
        bSourceID               5
        iTerminal               0 
    Interface Descriptor:
      bLength                 9
      bDescriptorType         4
      bInterfaceNumber        1
      bAlternateSetting       0
      bNumEndpoints           0
      bInterfaceClass         1 Audio
      bInterfaceSubClass      2 Streaming
      bInterfaceProtocol      0 
      iInterface              0 
    Interface Descriptor:
      bLength                 9
      bDescriptorType         4
      bInterfaceNumber        1
      bAlternateSetting       1
      bNumEndpoints           1
      bInterfaceClass         1 Audio
      bInterfaceSubClass      2 Streaming
      bInterfaceProtocol      0 
      iInterface              0 
      AudioStreaming Interface Descriptor:
        bLength                 7
        bDescriptorType        36
        bDescriptorSubtype      1 (AS_GENERAL)
        bTerminalLink           1
        bDelay                  1 frames
        wFormatTag         0x0001 PCM
      AudioStreaming Interface Descriptor:
        bLength                11
        bDescriptorType        36
        bDescriptorSubtype      2 (FORMAT_TYPE)
        bFormatType             1 (FORMAT_TYPE_I)
        bNrChannels             2
        bSubframeSize           2
        bBitResolution         16
        bSamFreqType            1 Discrete
        tSamFreq[ 0]        32000
      Endpoint Descriptor:
        bLength                 9
        bDescriptorType         5
        bEndpointAddress     0x01  EP 1 OUT
        bmAttributes            9
          Transfer Type            Isochronous
          Synch Type               Adaptive
          Usage Type               Data
        wMaxPacketSize     0x0084  1x 132 bytes
        bInterval               1
        bRefresh                0
        bSynchAddress           0
        AudioStreaming Endpoint Descriptor:
          bLength                 7
          bDescriptorType        37
          bDescriptorSubtype      1 (EP_GENERAL)
          bmAttributes         0x00
          bLockDelayUnits         0 Undefined
          wLockDelay         0x0000
    Interface Descriptor:
      bLength                 9
      bDescriptorType         4
      bInterfaceNumber        2
      bAlternateSetting       0
      bNumEndpoints           0
      bInterfaceClass         1 Audio
      bInterfaceSubClass      2 Streaming
      bInterfaceProtocol      0 
      iInterface              0 
    Interface Descriptor:
      bLength                 9
      bDescriptorType         4
      bInterfaceNumber        2
      bAlternateSetting       1
      bNumEndpoints           1
      bInterfaceClass         1 Audio
      bInterfaceSubClass      2 Streaming
      bInterfaceProtocol      0 
      iInterface              0 
      AudioStreaming Interface Descriptor:
        bLength                 7
        bDescriptorType        36
        bDescriptorSubtype      1 (AS_GENERAL)
        bTerminalLink           6
        bDelay                  1 frames
        wFormatTag         0x0001 PCM
      AudioStreaming Interface Descriptor:
        bLength                11
        bDescriptorType        36
        bDescriptorSubtype      2 (FORMAT_TYPE)
        bFormatType             1 (FORMAT_TYPE_I)
        bNrChannels             1
        bSubframeSize           2
        bBitResolution         16
        bSamFreqType            1 Discrete
        tSamFreq[ 0]        16000
      Endpoint Descriptor:
        bLength                 9
        bDescriptorType         5
        bEndpointAddress     0x82  EP 2 IN
        bmAttributes            5
          Transfer Type            Isochronous
          Synch Type               Asynchronous
          Usage Type               Data
        wMaxPacketSize     0x0022  1x 34 bytes
        bInterval               1
        bRefresh                0
        bSynchAddress           0
        AudioStreaming Endpoint Descriptor:
          bLength                 7
          bDescriptorType        37
          bDescriptorSubtype      1 (EP_GENERAL)
          bmAttributes         0x00
          bLockDelayUnits         0 Undefined
          wLockDelay         0x0000
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
          wDescriptorLength     507
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
        bInterval               5
      Endpoint Descriptor:
        bLength                 7
        bDescriptorType         5
        bEndpointAddress     0x03  EP 3 OUT
        bmAttributes            3
          Transfer Type            Interrupt
          Synch Type               None
          Usage Type               Data
        wMaxPacketSize     0x0040  1x 64 bytes
        bInterval               5
can't get device qualifier: Resource temporarily unavailable
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
4. optional: [convert to C Array](https://eleccelerator.com/usbdescreqparser/) (only hex values, dont input first output line)

### PS5 Gamepad via usb
```Shell
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

```C
0x05, 0x01,        // Usage Page (Generic Desktop Ctrls)
0x09, 0x05,        // Usage (Game Pad)
0xA1, 0x01,        // Collection (Application)
0x85, 0x01,        //   Report ID (1)
0x09, 0x30,        //   Usage (X)
0x09, 0x31,        //   Usage (Y)
0x09, 0x32,        //   Usage (Z)
0x09, 0x35,        //   Usage (Rz)
0x09, 0x33,        //   Usage (Rx)
0x09, 0x34,        //   Usage (Ry)
0x15, 0x00,        //   Logical Minimum (0)
0x26, 0xFF, 0x00,  //   Logical Maximum (255)
0x75, 0x08,        //   Report Size (8)
0x95, 0x06,        //   Report Count (6)
0x81, 0x02,        //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
0x06, 0x00, 0xFF,  //   Usage Page (Vendor Defined 0xFF00)
0x09, 0x20,        //   Usage (0x20)
0x95, 0x01,        //   Report Count (1)
0x81, 0x02,        //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
0x05, 0x01,        //   Usage Page (Generic Desktop Ctrls)
0x09, 0x39,        //   Usage (Hat switch)
0x15, 0x00,        //   Logical Minimum (0)
0x25, 0x07,        //   Logical Maximum (7)
0x35, 0x00,        //   Physical Minimum (0)
0x46, 0x3B, 0x01,  //   Physical Maximum (315)
0x65, 0x14,        //   Unit (System: English Rotation, Length: Centimeter)
0x75, 0x04,        //   Report Size (4)
0x95, 0x01,        //   Report Count (1)
0x81, 0x42,        //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,Null State)
0x65, 0x00,        //   Unit (None)
0x05, 0x09,        //   Usage Page (Button)
0x19, 0x01,        //   Usage Minimum (0x01)
0x29, 0x0F,        //   Usage Maximum (0x0F)
0x15, 0x00,        //   Logical Minimum (0)
0x25, 0x01,        //   Logical Maximum (1)
0x75, 0x01,        //   Report Size (1)
0x95, 0x0F,        //   Report Count (15)
0x81, 0x02,        //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
0x06, 0x00, 0xFF,  //   Usage Page (Vendor Defined 0xFF00)
0x09, 0x21,        //   Usage (0x21)
0x95, 0x0D,        //   Report Count (13)
0x81, 0x02,        //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
0x06, 0x00, 0xFF,  //   Usage Page (Vendor Defined 0xFF00)
0x09, 0x22,        //   Usage (0x22)
0x15, 0x00,        //   Logical Minimum (0)
0x26, 0xFF, 0x00,  //   Logical Maximum (255)
0x75, 0x08,        //   Report Size (8)
0x95, 0x34,        //   Report Count (52)
0x81, 0x02,        //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
0x85, 0x02,        //   Report ID (2)
0x09, 0x23,        //   Usage (0x23)
0x95, 0x2F,        //   Report Count (47)
0x91, 0x02,        //   Output (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0x05,        //   Report ID (5)
0x09, 0x33,        //   Usage (0x33)
0x95, 0x28,        //   Report Count (40)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0x08,        //   Report ID (8)
0x09, 0x34,        //   Usage (0x34)
0x95, 0x2F,        //   Report Count (47)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0x09,        //   Report ID (9)
0x09, 0x24,        //   Usage (0x24)
0x95, 0x13,        //   Report Count (19)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0x0A,        //   Report ID (10)
0x09, 0x25,        //   Usage (0x25)
0x95, 0x1A,        //   Report Count (26)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0x20,        //   Report ID (32)
0x09, 0x26,        //   Usage (0x26)
0x95, 0x3F,        //   Report Count (63)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0x21,        //   Report ID (33)
0x09, 0x27,        //   Usage (0x27)
0x95, 0x04,        //   Report Count (4)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0x22,        //   Report ID (34)
0x09, 0x40,        //   Usage (0x40)
0x95, 0x3F,        //   Report Count (63)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0x80,        //   Report ID (-128)
0x09, 0x28,        //   Usage (0x28)
0x95, 0x3F,        //   Report Count (63)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0x81,        //   Report ID (-127)
0x09, 0x29,        //   Usage (0x29)
0x95, 0x3F,        //   Report Count (63)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0x82,        //   Report ID (-126)
0x09, 0x2A,        //   Usage (0x2A)
0x95, 0x09,        //   Report Count (9)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0x83,        //   Report ID (-125)
0x09, 0x2B,        //   Usage (0x2B)
0x95, 0x3F,        //   Report Count (63)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0x84,        //   Report ID (-124)
0x09, 0x2C,        //   Usage (0x2C)
0x95, 0x3F,        //   Report Count (63)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0x85,        //   Report ID (-123)
0x09, 0x2D,        //   Usage (0x2D)
0x95, 0x02,        //   Report Count (2)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0xA0,        //   Report ID (-96)
0x09, 0x2E,        //   Usage (0x2E)
0x95, 0x01,        //   Report Count (1)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0xE0,        //   Report ID (-32)
0x09, 0x2F,        //   Usage (0x2F)
0x95, 0x3F,        //   Report Count (63)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0xF0,        //   Report ID (-16)
0x09, 0x30,        //   Usage (0x30)
0x95, 0x3F,        //   Report Count (63)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0xF1,        //   Report ID (-15)
0x09, 0x31,        //   Usage (0x31)
0x95, 0x3F,        //   Report Count (63)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0xF2,        //   Report ID (-14)
0x09, 0x32,        //   Usage (0x32)
0x95, 0x0F,        //   Report Count (15)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0xF4,        //   Report ID (-12)
0x09, 0x35,        //   Usage (0x35)
0x95, 0x3F,        //   Report Count (63)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0x85, 0xF5,        //   Report ID (-11)
0x09, 0x36,        //   Usage (0x36)
0x95, 0x03,        //   Report Count (3)
0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
0xC0,              // End Collection

// 273 bytes
```


