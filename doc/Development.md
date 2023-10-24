### Allow docker to run images with a different target platform than the host
- `docker run --rm --privileged multiarch/qemu-user-static --reset -p yes`
- This links all platforms supported by qemu correctly for docker
- This might have to be run after every host system reboot

### Usage
All commands run in project root

1. Develop on your host architecture: `docker compose up develop`
    - This keeps the container open in the current terminal, after you are done working you can stop it with Ctrl + C
2. Build the binary for aarch64/arm64: 
   - `./buildAarch64.sh` - this increments the build number in Cargo + builds the binary and copies it into aarch64/ to allow git tracking + removes container after its finished
      - Why increment the build number in Cargo.toml?  
      - Because sometimes the combination of docker caching and cargo build doesnt use the most up-to-date Cargo.toml. <br>
      This way the Cargo.toml always changes and wont be cached
   - `docker compose up aarch64-build` - only builds binary into aarch64/target/

### dmesg errors on linux 6.1
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

### Possiblity for multiple HID devices with one connection:
https://electronics.stackexchange.com/a/400268

### XBOX
[XBOX Dev Wiki](https://xboxdevwiki.net/Xbox_Input_Devices#Standard_Gamepads)

### PS
[PS Dev Wiki](https://www.psdevwiki.com/)
[DualShock Dev Wiki](https://www.psdevwiki.com/ps4/DualShock_4)
[PS4 HID Output Report BT](https://www.psdevwiki.com/ps4/DS4-BT#HID_OUTPUT_reports)

### Random Ideas and Inspiration
- [Linux BT UI written in Rust](https://github.com/kaii-lb/overskride)
- [RPI Embedded](https://docs.rs/rpi_embedded/latest/rpi_embedded/)
    - Fork of [RPPAL](https://github.com/golemparts/rppal)
    - RPi still runs linux, but this library apparently allows direct access to hardware components
    - If this works, could be used for better USB output
