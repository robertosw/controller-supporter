# Short Explanation
- USB connections consist of a host and a device (client / gadget)
- Most computers only have usb host capabilities. 
  Meaning you can plug in a usb device (e.g. a keyboard) or some storage, and your computer controls what this device is allowed to do
- Most microcontrollers and embedded systems also provide gadget capabilities.
  Meaning this microcontroller can be recognized by some host as e.g. a keyboard
  - The Raspberry Pi Zero 2 has these capabilities, but they have to be enabled in the boot config of linux

<br>

# 1. Enable USB Gadget capabilities
More info can be found here:
- [Turn Your Raspberry Pi Zero into a USB Keyboard (HID)](https://randomnerdtutorials.com/raspberry-pi-zero-usb-keyboard-hid/)
- [Composite usb gadgets on the raspberry pi zero](https://www.isticktoit.net/?p=1383)

<br>

Tell linux to use the hardware capabilities of the raspi:
- `$ sudo nano /boot/config.txt`  Open the boot config
  - Search for this block:
    ```Shell
    [cm4]
    # Enable host mode on the 2711 built-in XHCI USB controller.
    # This line should be removed if the legacy DWC2 controller is required
    # (e.g. for USB device mode) or if USB support is not required.
    otg_mode=1
    ```
    and uncomment the first and last line:
    ```Shell
    # [cm4]
    # Enable host mode on the 2711 built-in XHCI USB controller.
    # This line should be removed if the legacy DWC2 controller is required
    # (e.g. for USB device mode) or if USB support is not required.
    # otg_mode=1
    ```

  - At the end of the file add:
    ```Shell
    [all]
    dtoverlay=dwc2,dr_mode=peripheral
    ```
  - Save the file with Ctrl + O, exit with Ctrl + X and reboot immediately (`$ sudo reboot`)
  - You can find out more about what device overlays are if you read the `/boot/overlays/README.txt`
- Add the modules `dwc2` and `libcomposite` to `/etc/modules`:
  - `$ echo "dwc2" | sudo tee -a /etc/modules`
  - `$ sudo echo "libcomposite" | sudo tee -a /etc/modules`


<br>

# 2. Set root permissions
- This program modifies files inside `/sys/kernel/config/usb_gadget` which are write protected by default
- These files contain the configuration to emulate the Raspberry Pi as an USB Device once connected to an host
- To allow this programm to change the values set the owner of the file as root and set the setuid permission:
  - `sudo chown root:root ./gamgepad_bridge`
  - `sudo chmod +s ./gamgepad_bridge`
- This does include some security risk. If some part of this program can be exploited, an attacker can run other code as the root user and effectively do anything with your Raspberry Pi. Since this requires someone to already have physical access or some way of login to your Raspberry Pi this risk is accepted.
  - If you know a better way to allow the modification of these files while still being able to run this program from autostart without any manual action, please let me know.
