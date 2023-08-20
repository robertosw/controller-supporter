# Short Explanation
- USB connections consist of a host and a device (client / gadget)
- Most computers only have usb host capabilities. 
  Meaning you can plug in a usb device (e.g. a keyboard) or some storage, and your computer controls what this device is allowed to do
- Most microcontrollers and embedded systems also provide gadget capabilities.
  Meaning this microcontroller can be recognized by some host as e.g. a keyboard
  - The Raspberry Pi Zero 2 has these capabilities, but they have to be enabled in the boot config of linux

# Enable USB Gadget capabilities
More info can be found here:
- [Turn Your Raspberry Pi Zero into a USB Keyboard (HID)](https://randomnerdtutorials.com/raspberry-pi-zero-usb-keyboard-hid/)
- [Composite usb gadgets on the raspberry pi zero](https://www.isticktoit.net/?p=1383)

<br>

Tell linux to use the hardware capabilities of the raspi:
- `$ echo "dtoverlay=dwc2" | sudo tee -a /boot/config.txt`
- `$ echo "dwc2" | sudo tee -a /etc/modules`
- `$ sudo echo "libcomposite" | sudo tee -a /etc/modules`

