If you have any ideas or know of anything I should avoid or be careful with, don't hesitate to create a discussion to tell me :D

# Current state

- Bluetooth connection was manually established, but some code already exists for automatic detection
- Reading and understanding all buttons from bluetooth-connected dual sense (ps5) gamepad works
- configuring the RPi as a gadget *seems* to work
  - Linux detects (`lsusb`) the RPi as the simulated gamepad, but `dmesg` shows some errors that were not shown on previous linux kernels (I recently switched my distro and with this from 5.15 to 6.1)
  - `dmesg` errors:
    ```
    usb 1-2: new high-speed USB device number 10 using xhci_hcd
    usb 1-2: New USB device found, idVendor=054c, idProduct=0ce6, bcdDevice= 1.00
    usb 1-2: New USB device strings: Mfr=1, Product=2, SerialNumber=3
    usb 1-2: Product: Wireless Controller
    usb 1-2: Manufacturer: Sony Interactive Entertainment
    playstation 0003:054C:0CE6.0013: hidraw4: USB HID v1.01 Gamepad [Sony Interactive Entertainment Wireless Controller] on usb-0000:03:00.3-2/input0
    playstation 0003:054C:0CE6.0013: Invalid reportID received, expected 9 got 0
    playstation 0003:054C:0CE6.0013: Failed to retrieve DualSense pairing info: -22
    playstation 0003:054C:0CE6.0013: Failed to get MAC address from DualSense
    playstation 0003:054C:0CE6.0013: Failed to create dualsense.
    playstation: probe of 0003:054C:0CE6.0013 failed with error -22
    ```
    - These might be fixable if I actually create all audio functions the real controller has, but I dont think thats the problem
  - Additionally when trying to write to the file `/dev/hidg0` rust tells me: <br>
  `code: 108, kind: Uncategorized, message: "Cannot send after transport endpoint shutdown"`
    - This is the same error that appears when the RPi is not yet connected to a host
    - This suggests that this file doesnt work as I think
    - I couldnt find any other sources that show other files which are used as device files for output data

<br>

# Idea


Personally, I enjoy playing with PlayStation controllers, simply because of their almost symmetrical layout.
The problem is that only one platform supports one of the PS controllers. Only the PS4 natively supports the PS4 controller, and only the PS5 natively supports the PS5 controller. 
With this project I want to solve this problem by using the raspi in between to translate the used controller for the connected platform.
I expect there will be some problems as Playstations communicate both ways, but maybe this can be ignored...


# What I can test
- **Platforms:** PS4 and PC
- **Controllers:** PS5, PS4 connected via BT to Raspi
 
# Used hardware
- Raspberry Pi Zero 2W
  - Since this raspi is able to run the normal raspi OS, this project is made easier because the bluetooth and usb connection is handled by the OS, which means the data can just be read from and written to a filehandler instead of having to connect everything manually.
- [Raspberry Sense HAT (not yet)](https://www.raspberrypi.com/products/sense-hat)
- Emulator stick
  - Connects to PS5 controllers and emulates them as XBOX for Win10 and PS4 for PS4
  - Why don't I just use this?
    - When playing coop with a friend on a PS4, this stick often loses connection with my PS5 controller <br>
(The stick or controller is not faulty, because when playing alone the connection lasts until the controller battery dies)

# Goal
Due to the PS4 problem described in "Used Hardware", the Raspi must be connected to the targeted platform via a USB cable.

Ideally, the Raspi would support a Bluetooth connection to two controllers, processing both inputs and translating them to the platform. Since USB allows a lot of things in one connection this shouldn't really be a problem :D

# Possible Inspirations / Mapping Sources
- [GP2040](https://github.com/FeralAI/GP2040)
- [GP2040-CE](https://github.com/OpenStickCommunity/GP2040-CE)
- [Passing Link](https://github.com/passinglink/passinglink)

Why dont I just use these?
1. I want to learn how this can be done
2. Both firmwares dont support PS5 controllers
